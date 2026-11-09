use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, routing::post};
use tokio::net::TcpListener;

use crate::{db::DB, query_engine::Query};

mod db;
mod query_engine;
mod user_service;

async fn gql_handler(gql_req: GraphQLRequest) -> GraphQLResponse {
    let query = Query { db: DB };
    let schema = Schema::new(query, EmptyMutation, EmptySubscription);
    let response = schema.execute(gql_req.into_inner()).await;

    response.into()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/graphql", post(gql_handler));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running...");
    axum::serve(listener, app).await.unwrap();
}
