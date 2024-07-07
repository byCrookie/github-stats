# github-stats

a low memory api in rust to generate github stats

# Environment

Use environement variables or .env file to supply values.

| Variable            | Required | Default           | Description                                                                                                                                 |
| ------------------- | -------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| STATS_GITHUB_USER   | yes      |                   |                                                                                                                                             |
| STATS_GITHUB_TOKEN  | yes      |                   | github classic token with repo scope (returns more commits) or github fine-grained token with contents (read-only) and metadata (read-only) |
| STATS_CACHE_SECONDS | no       | 86400             |                                                                                                                                             |
| STATS_BASE_URL      | no       | empty             |                                                                                                                                             |
| STATS_CACHE_PATH    | no       | working directory |                                                                                                                                             |
| RUST_LOG            | no       | info              |                                                                                                                                             |
