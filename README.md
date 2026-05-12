# github-stats

A streamlined version of [anuraghazra/github-readme-stats](https://github.com/anuraghazra/github-readme-stats)
implemented in rust to reduce resource
consumption. This implementation uses filesystem caching to minimize memory usage.

# Example

![byCrookie's Github Stats](https://github-stats.bycrookie.com/combined?lang_count=40)

# Usage

If the application is running, open a browser and navigate to the root of the application. The application will return
a json view of all possible endpoints.

# Self-hosting

It is recommended to use docker or podman to self-host the application. Follow the instructions in
the [Docker / Podman](#docker--podman) section.

# Environment

Use environment variables or .env file to supply values. An example .env file is provided
in [.env.example](.env.example).

| Variable                   | Required | Default           | Description                                                                                                                                 |
|----------------------------|----------|-------------------|---------------------------------------------------------------------------------------------------------------------------------------------|
| STATS_GITHUB_USER          | yes      |                   | GitHub username                                                                                                                             |
| STATS_GITHUB_TOKEN         | yes      |                   | GitHub classic token with repo scope (returns more commits) or github fine-grained token with contents (read-only) and metadata (read-only) |
| STATS_CACHE_SECONDS        | no       | 86400             | How long content is served from the cache.                                                                                                  |
| STATS_BASE_URL             | no       | empty             | Is used to return fully qualified urls in rest responses.                                                                                   |
| STATS_IPV4_ADDRESS         | no       |                   | IPv4 address to listen on. Does not include the port. At least on of IPv4/IPv6 has to be supplied.                                          |
| STATS_IPV6_ADDRESS         | no       |                   | IPv6 address to listen on. Does not include the port. At least on of IPv4/IPv6 has to be supplied.                                          |
| STATS_PORT                 | no       | 8080              | Port to listen on. Is combined with the supplied addresses.                                                                                 |
| STATS_CACHE_PATH           | no       | working directory | Folder where caches are stored.                                                                                                             |
| STATS_IGNORED_REPOSITORIES | no       |                   | Ignore repositories by their full name {user}/{repo}. Does not affect total stars and commits. Only affects language percentages.           |
| RUST_LOG                   | no       | info              | How detailed the log messages are.                                                                                                          |

# Docker / Podman

Use the [Containerfile](Containerfile) to build a container image or use the
example [compose.yaml.example](compose.yaml.example) for compose. The rust application is compiled with the `--release`
flag
and run in a minimal container. Do not forget to supply the environment variables.
