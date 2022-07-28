use sqlx::{FromRow, Row, query, SqlitePool, sqlite::SqliteRow};
use async_graphql::{
    Context, Object, Schema, EmptySubscription,
};

pub type GQLSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(Clone, FromRow)]
pub struct Entry {
    id: i32,
    email: String,
    pubkey: String,
}

#[Object]
impl Entry {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn email(&self) -> String {
        self.email.clone()
    }

    async fn pubkey(&self) -> String {
        self.pubkey.clone()
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn entries (&self, ctx: &Context<'_>,) -> Vec<Entry> {
        let pool = ctx.data_unchecked::<SqlitePool>();
        query(
            r#"
            select id, email, pubkey from key;
            "#)
            .map(|row: SqliteRow| {
                Entry {
                    id: row.get(0),
                    email: row.get(1),
                    pubkey: row.get(2),
                }
            })
            .fetch_all(pool)
            .await
            .unwrap_or_default()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_entry(
        &self, ctx: &Context<'_>, email: String, pubkey: String)
        -> i32 {
            let pool = ctx.data_unchecked::<SqlitePool>();
            let res = query(
                r#"
                insert into key
                (email, pubkey)
                values ($1, $2)
                returning id;
                "#
                )
                .bind(email)
                .bind(pubkey)
                .fetch_one(pool)
                .await;
            match res {
                Ok(r) => r.get(0),
                Err(e) => {
                    println!("{e}");
                    -1
                }
            }
        }
}
