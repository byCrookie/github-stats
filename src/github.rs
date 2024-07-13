use std::collections::HashMap;
use std::time::{Duration, UNIX_EPOCH};

use anyhow::anyhow;
use log::{debug, error};
use reqwest::Client;
use reqwest::header::{
    ACCEPT, AUTHORIZATION, HeaderMap, HeaderName, HeaderValue, LINK, USER_AGENT,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::time::sleep;
use url::Url;

const MAX_RETRIES: u32 = 2;
const INITIAL_BACKOFF: u64 = 2;

#[derive(Deserialize, Serialize, Debug)]
pub struct Stats {
    pub total_stars: u32,
    pub total_commits: u32,
    pub languages: HashMap<String, Language>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Repository {
    id: u32,
    name: String,
    full_name: String,
    owner: Owner,
    stargazers_count: u32,
    languages_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Owner {
    login: String,
    id: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Commit {
    url: String,
    sha: String,
    commit: CommitDetail,
}

#[derive(Deserialize, Serialize, Debug)]
struct CommitDetail {
    author: CommitAuthor,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct CommitAuthor {
    name: String,
    date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Language {
    pub name: String,
    pub color: String,
    pub size: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct SearchResult<T> {
    total_count: u32,
    incomplete_results: bool,
    items: Vec<T>,
}

async fn make_github_request(
    client: &Client,
    url: &str,
) -> Result<reqwest::Response, anyhow::Error> {
    let mut retries = 0;
    let mut backoff = INITIAL_BACKOFF;

    loop {
        let response = client.get(url).send().await;

        match response {
            Ok(resp) => {
                let headers = resp.headers().clone();
                if resp.status() == reqwest::StatusCode::OK {
                    if let Some(rate_limit) = ratelimit_remaining(&headers) {
                        if rate_limit == 0 {
                            debug!("Primary rate limit exceeded. Rate limit remaining is zero");
                            if let Some(retry_after) = ratelimit_reset(&headers) {
                                debug!(
                                    "Primary rate limit exceeded, sleeping for {} seconds",
                                    retry_after
                                );
                                sleep(Duration::from_secs(retry_after)).await;
                                continue;
                            }
                        }
                    }
                }

                if let Some(retry_after) = retry_after(&headers) {
                    debug!(
                        "Secondary rate limit exceeded, sleeping for {} seconds",
                        retry_after
                    );
                    sleep(Duration::from_secs(retry_after)).await;
                    continue;
                }

                let retry_http_codes: Vec<reqwest::StatusCode> = vec![
                    reqwest::StatusCode::REQUEST_TIMEOUT,
                    reqwest::StatusCode::TOO_MANY_REQUESTS,
                    reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    reqwest::StatusCode::BAD_GATEWAY,
                    reqwest::StatusCode::SERVICE_UNAVAILABLE,
                    reqwest::StatusCode::GATEWAY_TIMEOUT,
                ];

                if resp.status().is_success() {
                    return Ok(resp);
                } else if retry_http_codes.contains(&resp.status()) {
                    debug!("Request failed with status: {}. Retrying...", resp.status());
                } else {
                    return Err(anyhow!("Request failed with status: {}.", resp.status()));
                }
            }
            Err(err) => {
                return Err(anyhow!("Failed to send request: {err}"));
            }
        }

        if retries >= MAX_RETRIES {
            return Err(anyhow!("Max retries of {MAX_RETRIES} reached"));
        }

        retries += 1;
        sleep(Duration::from_secs(backoff)).await;
        backoff *= 2;
    }
}

fn ratelimit_remaining(headers: &HeaderMap) -> Option<u64> {
    if let Some(rate_limit_remaining_header) = headers.get("x-ratelimit-remaining") {
        if let Ok(rate_limit_remaining) = rate_limit_remaining_header.to_str() {
            if let Ok(rate_limit) = rate_limit_remaining.parse::<u64>() {
                return Some(rate_limit);
            }
        }
    }
    None
}

fn ratelimit_reset(headers: &HeaderMap) -> Option<u64> {
    if let Some(rate_limit_reset_header) = headers.get("x-ratelimit-reset") {
        if let Ok(rate_limit_reset) = rate_limit_reset_header.to_str() {
            if let Ok(reset_timestamp) = rate_limit_reset.parse::<u64>() {
                if let Ok(now) = std::time::SystemTime::now().duration_since(UNIX_EPOCH) {
                    let now_secs = now.as_secs();
                    if reset_timestamp > now_secs {
                        return Some(reset_timestamp - now_secs);
                    }
                }
            }
        }
    }
    None
}

fn retry_after(headers: &HeaderMap) -> Option<u64> {
    if let Some(retry_after_header) = headers.get("retry-after") {
        if let Ok(retry_after) = retry_after_header.to_str() {
            if let Ok(retry_after_seconds) = retry_after.parse::<u64>() {
                return Some(retry_after_seconds);
            }
        }
    }
    None
}

async fn fetch_all_pages<T: DeserializeOwned>(
    client: &Client,
    initial_url: &str,
) -> Result<SearchResult<T>, anyhow::Error> {
    debug!("Fetch all pages for {initial_url}");
    let mut total_count = 0;
    let mut all_items = Vec::new();
    let mut next_url = Some(Url::parse(initial_url)?);

    while let Some(url) = next_url.take() {
        debug!("Call {}", url.as_str());
        let response = make_github_request(&client, url.as_str()).await?;
        if response.status().is_success() {
            let headers = response.headers().clone();
            debug!("Headers {:?}", headers);
            let result: SearchResult<T> = response.json().await?;

            if result.incomplete_results {
                return Err(anyhow!("Fetch was incomplete"));
            }

            debug!("Total Count {}", result.total_count);
            debug!("Count {}", result.items.len());
            if total_count <= result.total_count {
                total_count = result.total_count;
            }
            all_items.extend(result.items);
            next_url = parse_next_url(&headers)?;
            debug!("Next {:?}", next_url);
        } else {
            return Err(anyhow!("Failed to fetch data: {}", response.status()));
        }
    }

    Ok(SearchResult {
        total_count,
        incomplete_results: false,
        items: all_items,
    })
}

fn parse_next_url(headers: &HeaderMap) -> Result<Option<Url>, anyhow::Error> {
    if let Some(link_header) = headers.get(LINK) {
        let link_str = link_header.to_str()?;
        let links = parse_link_header(link_str)?;
        debug!("Links {:?}", links);
        if let Some(next) = links.get("next") {
            return Ok(Some(next.clone()));
        }
    }

    Ok(None)
}

fn parse_link_header(header: &str) -> Result<HashMap<String, Url>, anyhow::Error> {
    let mut links = HashMap::new();
    for link in header.split(',') {
        let parts: Vec<&str> = link.split(';').collect();
        if parts.len() == 2 {
            if let Ok(url) = Url::parse(parts[0].replace("<", "").replace(">", "").trim()) {
                if let Some(rel) = parts[1].split('=').nth(1) {
                    let rel = rel.trim_matches('"').trim();
                    links.insert(rel.to_string(), url);
                }
            } else {
                return Err(anyhow!("Failed to parse link in link header: {}", parts[0]));
            }
        } else {
            return Err(anyhow!(
                "Link in link header as more than two parts: {:?}",
                parts
            ));
        }
    }
    Ok(links)
}

impl Stats {
    pub async fn request(github_user: &str, github_token: &str) -> Result<Self, anyhow::Error> {
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
        headers.insert(USER_AGENT, HeaderValue::from_str(&github_user)?);

        let client = Client::builder()
            .default_headers(headers)
            .connection_verbose(true)
            .http1_title_case_headers()
            .build()?;

        let repo_result: SearchResult<Repository> = fetch_all_pages(
            &client,
            &format!(
                "https://api.github.com/search/repositories?q=user:{github_user}&per_page=100"
            ),
        )
            .await?;

        let total_stars = repo_result
            .items
            .iter()
            .map(|repo| repo.stargazers_count)
            .sum();

        let commit_result: SearchResult<Commit> = make_github_request(
            &client,
            &format!("https://api.github.com/search/commits?q=author:{github_user}"),
        )
            .await?
            .json()
            .await?;

        let total_commits = commit_result.total_count;

        let mut languages: HashMap<String, Language> = HashMap::new();
        let colors: HashMap<String, String> = crate::language_colors::colors();
        for repo in repo_result.items.iter() {
            let langs: HashMap<String, f64> = make_github_request(&client, &repo.languages_url)
                .await?
                .json()
                .await?;

            for lang in langs.iter() {
                if languages.contains_key(lang.0) {
                    let existing: &Language = languages.get(lang.0).unwrap();
                    languages.insert(
                        lang.0.clone(),
                        Language {
                            color: existing.color.clone(),
                            name: existing.name.clone(),
                            size: existing.size + lang.1,
                        },
                    );
                    continue;
                }

                let default = String::from("#FBFF00");
                let color = colors.get(&lang.0.clone()).unwrap_or(&default);

                let language = Language {
                    color: color.clone(),
                    name: lang.0.clone(),
                    size: lang.1.clone(),
                };

                languages.insert(lang.0.clone(), language);
            }
        }

        let stats = Stats {
            total_stars,
            total_commits,
            languages,
        };

        debug!("{:#?}", stats);
        return Ok(stats);
    }
}

pub async fn request_stats(github_user: &str, github_token: &str) -> Result<Stats, anyhow::Error> {
    return match Stats::request(github_user, github_token).await {
        Ok(stats) => Ok(stats),
        Err(err) => {
            error!("{err}");
            Err(err)
        }
    };
}

#[allow(dead_code)]
pub async fn test(github_user: &str, github_token: &str) {
    let _ = request_stats(&github_user, &github_token).await.unwrap();
}
