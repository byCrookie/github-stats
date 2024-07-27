# github-stats

A streamlined version of [anuraghazra/github-readme-stats](https://github.com/anuraghazra/github-readme-stats)
implemented in rust to reduce resource
consumption. This implementation uses filesystem caching to minimize memory usage.

# Example

![byCrookie's Github Stats](https://github-stats.bycrookie.com/all)

# Usage

If the application is running, open a browser and navigate to the root of the application. The application will return
a json view of all possible endpoints.

# Self-hosting

It is recommended to use docker to self-host the application. Follow the instructions in the [Docker](#docker) section.

# Environment

Use environment variables or .env file to supply values.

| Variable            | Required | Default           | Description                                                                                                                                 |
|---------------------|----------|-------------------|---------------------------------------------------------------------------------------------------------------------------------------------|
| STATS_GITHUB_USER   | yes      |                   | GitHub username                                                                                                                             |
| STATS_GITHUB_TOKEN  | yes      |                   | GitHub classic token with repo scope (returns more commits) or github fine-grained token with contents (read-only) and metadata (read-only) |
| STATS_CACHE_SECONDS | no       | 86400             | How long content is served from the cache.                                                                                                  |
| STATS_BASE_URL      | no       | empty             | Is used to return fully qualified urls in rest responses.                                                                                   |
| STATS_IPV4_ADDRESS  | no       |                   | IPv4 address to listen on. Does not include the port. At least on of IPv4/IPv6 has to be supplied.                                          |
| STATS_IPV6_ADDRESS  | no       |                   | IPv6 address to listen on. Does not include the port. At least on of IPv4/IPv6 has to be supplied.                                          |
| STATS_PORT          | yes      |                   | Port to listen on. Is combined with the supplied addresses.                                                                                 |
| STATS_CACHE_PATH    | no       | working directory | Folder where caches are stored.                                                                                                             |
| RUST_LOG            | no       | info              | How detailed the log messages are.                                                                                                          |

# Docker

Use the [Dockerfile](./Dockerfile) to build a docker image. The rust application is compiled with the `--release` flag
and run in an alpine container. Do not forget to supply the environment variables.

## Donations

If you find github-stats useful and wish to support its development, donations are greatly appreciated.
You can make contributions in Ethereum (ETH) and Bitcoin (BTC) to the following addresses:

* ETH Address: 0x1C0416cC1DDaAEEb3017D4b8Dcd3f0B82f4d94C1
* BTC Address: bc1qygqya2w3hgpvy8hupctfkv5x06l69ydq4su2e2

## Stars

If you find github-stats beneficial and appreciate the effort that went into its creation, please consider
showing your support by giving the repository a star on GitHub.

## Contribute

We welcome contributions from the community to enhance github-stats. If you have suggestions for improvements or
discover any issues, please feel free to contribute to our project on GitHub. Your input is valuable in making this tool
even more versatile and efficient.

Thank you for choosing the github-stats 🚀
