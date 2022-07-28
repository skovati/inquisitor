# inquisitor

simple, fast, and reliable pgp keyserver.

utlizes [Sequoia](https://sequoia-pgp.org/) and exposes a [GraphQL](https://graphql.org) API to return rich data about keys.

## installation
```bash
cargo install --git https://github.com/skovati/inquisitor
```

## usage
```bash
inquisitor &
xdg-open http://localhost:8080/gql
```
or, from the command line
```bash
curl \
    --silent \
    --request POST \
    --header "Content-Type: application/json" \
    --data '{ "query": "{ search(email: \"sko\") { email pubkey } }"}' \
    http://localhost:8080/gql | jq
```

written in Rust using [axum](https://github.com/tokio-rs/axum), [async-graphql](https://github.com/async-graphql/async-graphql), [sqlx](https://github.com/launchbadge/sqlx), and [sqlite](https://sqlite.org).

