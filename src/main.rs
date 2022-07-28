use std::path::Path;

use anyhow::Result;
use async_graphql::{Schema, EmptySubscription};
use axum::{Router, Extension, routing::get};
use sqlx::sqlite::SqlitePool;
use sqlx::query;

mod resolver;
mod routes;

use crate::resolver::{Query, Mutation};

const DB_STR: &str = "db/inquisitor.db";

#[tokio::main]
async fn main() -> Result<()> {
    // // create sqlite database if it doesn't exist
    let path = Path::new(DB_STR);
    if !path.exists() {
        println!("creating empty database...");
        std::fs::File::create(path)?;
    }

    // connect to db
    let db_path = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| String::from(DB_STR));
    let pool = SqlitePool::connect(&db_path).await?;

    // and ensure schema exists
    let res = query(
        r#"
        create table if not exists key (
            id integer primary key autoincrement not null,
            email text,
            pubkey blob
        );
        "#
    )
    .execute(&pool)
    .await?;

    println!("ensuring schema, {} rows affected", res.rows_affected());

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool.clone())
        .finish();

    let app = Router::new()
        .route("/gql",
            get(routes::graphql_playground)
            .post(routes::graphql_handler))
        .route("/health", get(routes::health))
        .route("/", get(routes::index))
        .layer(Extension(schema));

    println!("serving playground @ http://localhost:8080/gql");

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    pool.close().await;
    Ok(())
}
