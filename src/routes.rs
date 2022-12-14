use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig}, Request, Response,
};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    http::StatusCode,
    Json,
};

use crate::resolver::GQLSchema;

pub async fn graphql_handler(
    schema: Extension<GQLSchema>,
    req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/gql")))
}

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn index() -> Html<&'static str> {
    Html(std::include_str!("../www/index.html"))
}
