# github-stats

A streamlined version of [anuraghazra/github-readme-stats](https://github.com/anuraghazra/github-readme-stats)
implemented in Rust to reduce resource consumption. Stats are fetched from the GitHub API and cached on disk
to minimise memory usage and API rate-limit pressure.

## Example

![byCrookie's Github Stats](https://github-stats.bycrookie.com/combined)

## Endpoints

| Method | Path        | Description                                                              |
|--------|-------------|--------------------------------------------------------------------------|
| GET    | `/`         | JSON listing of all endpoint URLs (uses `STATS_BASE_URL` for full URLs). |
| GET    | `/stats`    | SVG card with total stars and total commits.                             |
| GET    | `/languages`| SVG card with a top-languages breakdown.                                 |
| GET    | `/combined` | SVG card combining stats and top languages.                              |
| GET    | `/health`   | JSON health check. Returns `200 healthy` when the cache is fresh, `503 degraded` when the cache is missing or stale. |
| POST   | `/refresh`  | Invalidates the stats cache. Requires `Authorization: Bearer <STATS_REFRESH_TOKEN>`. Disabled when `STATS_REFRESH_TOKEN` is not set. |

### Query parameters (SVG card endpoints)

| Parameter    | Default | Description                                                       |
|--------------|---------|-------------------------------------------------------------------|
| `theme`      | `dark`  | Card colour theme. Supported values: `dark`, `light`.            |
| `width`      | `300`   | Card width in pixels. Clamped to `[50, 2000]`.                   |
| `lang_count` | `10`    | Number of languages to display. Clamped to `[1, 100]`. `/languages` and `/combined` only. |

## Self-hosting

It is recommended to use Docker or Podman to self-host the application.

```
# build locally (uses Containerfile)
podman compose up --build

# or pull a pre-built multi-arch image (amd64 + arm64)
# copy compose.yaml.example → compose.yaml, fill in your .env, then:
podman compose up
```

See [compose.yaml](compose.yaml) (local build) and [compose.yaml.example](compose.yaml.example) (pre-built image).

## Environment variables

Copy [.env.example](.env.example) to `.env` and fill in the required values.
Inline comments are **not** supported in `.env` — use `.env.example` as the documented reference.

| Variable                   | Required | Default     | Description                                                                                                                                    |
|----------------------------|----------|-------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| `STATS_GITHUB_USER`        | yes      |             | GitHub username to collect stats for.                                                                                                          |
| `STATS_GITHUB_TOKEN`       | yes      |             | GitHub classic token with `repo` scope, or a fine-grained token with **Contents** (read) and **Metadata** (read) permissions.                 |
| `STATS_CACHE_SECONDS`      | no       | `86400`     | How long (seconds) a cached response is considered fresh before the GitHub API is queried again.                                               |
| `STATS_CACHE_PATH`         | no       | _(cwd)_     | Directory where `stats_cache.json` is written.                                                                                                 |
| `STATS_BASE_URL`           | no       | _(empty)_   | Public base URL included in the `/` endpoint listing (e.g. `https://github-stats.example.com`).                                               |
| `STATS_IPV4_ADDRESS`       | no       | `0.0.0.0`   | IPv4 address to bind. Set to empty to disable IPv4. At least one of IPv4/IPv6 must be provided.                                               |
| `STATS_IPV6_ADDRESS`       | no       | _(empty)_   | IPv6 address to bind. Set to enable dual-stack (e.g. `::`). At least one of IPv4/IPv6 must be provided.                                       |
| `STATS_PORT`               | no       | `8080`      | Port to listen on (shared by both IPv4 and IPv6 bindings).                                                                                     |
| `STATS_IGNORED_REPOSITORIES` | no     | _(empty)_   | Comma-separated list of repositories to exclude from language stats (format: `user/repo`). Does not affect total stars or commits.             |
| `STATS_EXCLUDE_FORKS`      | no       | `false`     | Set to `true` to exclude forked repositories from all stats and language counts.                                                               |
| `STATS_REFRESH_TOKEN`      | no       | _(empty)_   | Bearer token required to call `POST /refresh`. Leave empty to disable the endpoint entirely.                                                   |
| `RUST_LOG`                 | no       | `info`      | Log verbosity. Valid values: `error`, `warn`, `info`, `debug`, `trace`.                                                                        |
