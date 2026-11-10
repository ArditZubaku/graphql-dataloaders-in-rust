use async_graphql::dataloader::DataLoader;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::{Router, extract::State, routing::post};

use tokio::net::TcpListener;

use crate::{db::DB, organization_loader::OrganizationLoader, query_engine::Query};

mod db;
mod organization_loader;
mod query_engine;
mod user_service;

type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn gql_handler(State(schema): State<AppSchema>, gql_req: GraphQLRequest) -> GraphQLResponse {
    let loader = DataLoader::new(OrganizationLoader::new(DB), tokio::spawn);

    let request = gql_req.into_inner().data(loader);

    schema.execute(request).await.into()
}

#[tokio::main]
async fn main() {
    let db = DB;

    // We create the DataLoader inside gql_handler, but the schema is created only once.
    // This gives us request-scoped caching.
    let schema = Schema::build(Query { db }, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/graphql", post(gql_handler))
        .with_state(schema);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Running on http://localhost:3000/graphql");

    axum::serve(listener, app).await.unwrap();
}
