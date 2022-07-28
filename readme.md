# inquisitor

simple, fast, and reliable pgp keyserver.

utlizes [Sequoia](https://sequoia-pgp.org/) and exposes a GraphQL API to return rich data about keys.

written in Rust using [axum](https://github.com/tokio-rs/axum), [async-graphql](https://github.com/async-graphql/async-graphql), [sqlx](https://github.com/launchbadge/sqlx), and [sqlite](https://sqlite.org).
