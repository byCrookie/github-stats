use std::collections::HashMap;
use std::time::{Duration, UNIX_EPOCH};

use anyhow::anyhow;
use log::{debug, warn};
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, LINK, USER_AGENT,
};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::time::sleep;
use url::Url;

const MAX_RETRIES: u32 = 2;
const INITIAL_BACKOFF: u64 = 2;
const REQUEST_TIMEOUT_SECS: u64 = 30;

#[derive(Deserialize, Serialize, Debug)]
pub struct Stats {
    pub total_stars: u32,
    pub total_commits: u32,
    pub languages: HashMap<String, Language>,
}

#[derive(Deserialize, Debug)]
struct Repository {
    full_name: String,
    stargazers_count: u32,
    languages_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Language {
    pub name: String,
    pub color: String,
    pub size: f64,
}

#[derive(Deserialize, Debug)]
struct SearchResult<T> {
    total_count: u32,
    incomplete_results: bool,
    items: Vec<T>,
}

#[derive(Deserialize)]
struct CommitCount {
    total_count: u32,
}

async fn make_github_request(
    client: &Client,
    url: &str,
) -> Result<reqwest::Response, anyhow::Error> {
    let mut retries = 0;
    let mut backoff = INITIAL_BACKOFF;

    loop {
        let response = match client.get(url).send().await {
            Ok(resp) => resp,
            Err(err) => {
                // Retry transient network errors (connect / timeout); fail fast on others.
                if retries < MAX_RETRIES && (err.is_connect() || err.is_timeout()) {
                    debug!("Transient network error (attempt {}): {}", retries + 1, err);
                    retries += 1;
                    sleep(Duration::from_secs(backoff)).await;
                    backoff *= 2;
                    continue;
                }
                return Err(anyhow!("Failed to send request: {err}"));
            }
        };

        let headers = response.headers().clone();
        let status = response.status();

        // Secondary rate limit: GitHub sends a Retry-After header.
        if let Some(secs) = retry_after(&headers) {
            if retries < MAX_RETRIES {
                debug!("Secondary rate limit, sleeping {}s", secs);
                sleep(Duration::from_secs(secs)).await;
                retries += 1;
                continue;
            }
            return Err(anyhow!(
                "Secondary rate limit persists after {MAX_RETRIES} retries"
            ));
        }

        // Primary rate limit: 403 with x-ratelimit-remaining: 0.
        if status == reqwest::StatusCode::FORBIDDEN {
            if ratelimit_remaining(&headers) == Some(0) {
                if let Some(reset_secs) = ratelimit_reset(&headers) {
                    if retries < MAX_RETRIES {
                        debug!(
                            "Primary rate limit exhausted, sleeping {}s until reset",
                            reset_secs
                        );
                        sleep(Duration::from_secs(reset_secs)).await;
                        retries += 1;
                        continue;
                    }
                }
            }
            return Err(anyhow!("Request forbidden (403)"));
        }

        if status.is_success() {
            return Ok(response);
        }

        let retry_codes = [
            reqwest::StatusCode::REQUEST_TIMEOUT,
            reqwest::StatusCode::TOO_MANY_REQUESTS,
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            reqwest::StatusCode::BAD_GATEWAY,
            reqwest::StatusCode::SERVICE_UNAVAILABLE,
            reqwest::StatusCode::GATEWAY_TIMEOUT,
        ];

        if retry_codes.contains(&status) && retries < MAX_RETRIES {
            debug!("Request failed with status: {}. Retrying...", status);
        } else if retry_codes.contains(&status) {
            return Err(anyhow!(
                "Max retries of {MAX_RETRIES} reached, last status: {status}"
            ));
        } else {
            return Err(anyhow!("Request failed with status: {status}"));
        }

        retries += 1;
        sleep(Duration::from_secs(backoff)).await;
        backoff *= 2;
    }
}

fn ratelimit_remaining(headers: &HeaderMap) -> Option<u64> {
    headers
        .get("x-ratelimit-remaining")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
}

fn ratelimit_reset(headers: &HeaderMap) -> Option<u64> {
    let reset_timestamp: u64 = headers
        .get("x-ratelimit-reset")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())?;
    let now = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();
    reset_timestamp.checked_sub(now)
}

fn retry_after(headers: &HeaderMap) -> Option<u64> {
    headers
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
}

async fn fetch_all_pages<T: DeserializeOwned>(
    client: &Client,
    initial_url: &str,
) -> Result<SearchResult<T>, anyhow::Error> {
    debug!("Fetching all pages for {initial_url}");
    let mut total_count = 0;
    let mut all_items = Vec::new();
    let mut next_url = Some(Url::parse(initial_url)?);

    while let Some(url) = next_url.take() {
        let response = make_github_request(client, url.as_str()).await?;
        let headers = response.headers().clone();
        let result: SearchResult<T> = response.json().await?;

        if result.incomplete_results {
            warn!(
                "GitHub search API returned incomplete results for {}; using partial data",
                url
            );
        }

        total_count = total_count.max(result.total_count);
        all_items.extend(result.items);
        next_url = parse_next_url(&headers)?;
    }

    Ok(SearchResult {
        total_count,
        incomplete_results: false,
        items: all_items,
    })
}

fn parse_next_url(headers: &HeaderMap) -> Result<Option<Url>, anyhow::Error> {
    if let Some(link_header) = headers.get(LINK) {
        let links = parse_link_header(link_header.to_str()?);
        if let Some(next) = links.get("next") {
            return Ok(Some(next.clone()));
        }
    }
    Ok(None)
}

fn parse_link_header(header: &str) -> HashMap<String, Url> {
    let mut links = HashMap::new();

    for link in header.split(',') {
        let link = link.trim();
        let mut parts = link.splitn(2, ';');
        let url_part = match parts.next() {
            Some(u) => u.trim(),
            None => continue,
        };
        let attrs = parts.next().unwrap_or("");
        let url_str = url_part
            .trim_start_matches('<')
            .trim_end_matches('>')
            .trim();

        match Url::parse(url_str) {
            Ok(url) => {
                for attr in attrs.split(';') {
                    if let Some(rel_val) = attr.trim().strip_prefix("rel=") {
                        let rel = rel_val.trim().trim_matches('"').trim();
                        if !rel.is_empty() {
                            links.insert(rel.to_string(), url.clone());
                        }
                        break;
                    }
                }
            }
            Err(err) => {
                debug!("Skipping malformed URL in Link header '{url_str}': {err}");
            }
        }
    }

    links
}

impl Stats {
    pub async fn request(
        github_user: &str,
        github_token: &str,
        ignored_repos: &str,
        exclude_forks: bool,
    ) -> Result<Self, anyhow::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {github_token}"))?,
        );
        headers.insert(
            HeaderName::from_static("x-github-api-version"),
            HeaderValue::from_static("2022-11-28"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_str(github_user)?);

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()?;

        let fork_filter = if exclude_forks { "+fork:false" } else { "" };
        let repo_result: SearchResult<Repository> = fetch_all_pages(
            &client,
            &format!(
                "https://api.github.com/search/repositories?q=user:{github_user}{fork_filter}&per_page=100"
            ),
        )
        .await?;

        let total_stars = repo_result.items.iter().map(|r| r.stargazers_count).sum();

        let commit_count: CommitCount = make_github_request(
            &client,
            &format!("https://api.github.com/search/commits?q=author:{github_user}"),
        )
        .await?
        .json()
        .await?;

        let ignored: Vec<String> = ignored_repos
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        let mut languages: HashMap<String, Language> = HashMap::new();
        let colors = crate::language_colors::colors();

        for repo in repo_result
            .items
            .iter()
            .filter(|r| !ignored.contains(&r.full_name.to_lowercase()))
        {
            let langs: HashMap<String, f64> = make_github_request(&client, &repo.languages_url)
                .await?
                .json()
                .await?;

            for (name, &size) in &langs {
                let color = colors
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| String::from("#FBFF00"));
                languages
                    .entry(name.clone())
                    .and_modify(|e| e.size += size)
                    .or_insert(Language {
                        color,
                        name: name.clone(),
                        size,
                    });
            }
        }

        Ok(Stats {
            total_stars,
            total_commits: commit_count.total_count,
            languages,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_link_header_basic() {
        let header = r#"<https://api.github.com/search/repositories?page=2>; rel="next", <https://api.github.com/search/repositories?page=5>; rel="last""#;
        let links = parse_link_header(header);
        assert_eq!(links.len(), 2);
        assert!(links.contains_key("next"));
        assert!(links.contains_key("last"));
    }

    #[test]
    fn parse_link_header_extra_attributes() {
        let header =
            r#"<https://api.github.com/search/repositories?page=2>; rel="next"; type="text/html""#;
        let links = parse_link_header(header);
        assert_eq!(links.len(), 1);
        assert!(links.contains_key("next"));
    }

    #[test]
    fn parse_link_header_malformed_url_is_skipped() {
        let links = parse_link_header(r#"<not a url>; rel="next""#);
        assert!(links.is_empty());
    }

    #[test]
    fn parse_link_header_empty() {
        assert!(parse_link_header("").is_empty());
    }

    #[test]
    fn ratelimit_remaining_parses() {
        let mut headers = HeaderMap::new();
        headers.insert("x-ratelimit-remaining", HeaderValue::from_static("42"));
        assert_eq!(ratelimit_remaining(&headers), Some(42));
    }

    #[test]
    fn ratelimit_remaining_missing_returns_none() {
        assert_eq!(ratelimit_remaining(&HeaderMap::new()), None);
    }

    #[test]
    fn ratelimit_remaining_non_numeric_returns_none() {
        let mut headers = HeaderMap::new();
        headers.insert("x-ratelimit-remaining", HeaderValue::from_static("abc"));
        assert_eq!(ratelimit_remaining(&headers), None);
    }

    #[test]
    fn retry_after_parses() {
        let mut headers = HeaderMap::new();
        headers.insert("retry-after", HeaderValue::from_static("60"));
        assert_eq!(retry_after(&headers), Some(60));
    }

    #[test]
    fn retry_after_missing_returns_none() {
        assert_eq!(retry_after(&HeaderMap::new()), None);
    }
}
