use anyhow::{anyhow, Ok};
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, LINK, USER_AGENT,
};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
struct Stats {
    total_stars: usize,
    total_commits: usize,
}

#[derive(Deserialize, Serialize, Debug)]
struct Repository {
    id: u64,
    name: String,
    full_name: String,
    owner: Owner,
    stargazers_count: usize,
}

#[derive(Deserialize, Serialize, Debug)]
struct Owner {
    login: String,
    id: u64,
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

#[derive(Deserialize, Serialize, Debug)]
struct SearchResult<T> {
    total_count: usize,
    incomplete_results: bool,
    items: Vec<T>,
}

async fn fetch_all_pages<T: DeserializeOwned>(
    client: &Client,
    initial_url: &str,
) -> Result<SearchResult<T>, anyhow::Error> {
    let mut all_items = Vec::new();
    let mut next_url = Some(Url::parse(initial_url)?);

    while let Some(url) = next_url.take() {
        println!("Call {}", url.as_str());
        let response = client.get(url.as_str()).send().await?;
        if response.status().is_success() {
            let headers = response.headers().clone();
            println!("Headers {:?}", headers);
            let result: SearchResult<T> = response.json().await?;

            if result.incomplete_results {
                return Err(anyhow!("Fetch was incomplete"));
            }

            println!("Count {}", result.items.len());
            all_items.extend(result.items);
            next_url = parse_next_url(&headers)?;
            println!("Next {:?}", next_url);
            println!();
        } else {
            return Err(anyhow!("Failed to fetch data: {}", response.status()));
        }
    }

    Ok(SearchResult {
        total_count: all_items.len(),
        incomplete_results: false,
        items: all_items,
    })
}

fn parse_next_url(headers: &HeaderMap) -> Result<Option<Url>, anyhow::Error> {
    if let Some(link_header) = headers.get(LINK) {
        let link_str = link_header.to_str()?;
        let links = parse_link_header(link_str)?;
        println!("Links {:?}", links);
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
            if let Result::Ok(url) = Url::parse(parts[0].replace("<", "").replace(">", "").trim()) {
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

        let client = reqwest::Client::builder()
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

        let commit_result: SearchResult<Commit> = fetch_all_pages(
            &client,
            &format!("https://api.github.com/search/commits?q=author:{github_user}&per_page=100"),
        )
        .await?;

        let total_commits = commit_result.total_count;

        let stats = Stats {
            total_stars: total_stars,
            total_commits: total_commits,
        };

        return Ok(stats);
    }
}

pub async fn test(github_user: &str, github_token: &str) -> Result<(), anyhow::Error> {
    let stats = Stats::request(github_user, github_token).await?;
    println!("{:#?}", stats);
    Ok(())
}
