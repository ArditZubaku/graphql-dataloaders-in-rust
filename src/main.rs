use async_graphql::dataloader::DataLoader;
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Router, extract::State};

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

async fn playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() {
    let db = DB;

    // We create the DataLoader inside gql_handler, but the schema is created only once.
    // This gives us request-scoped caching.
    let schema = Schema::build(Query { db }, EmptyMutation, EmptySubscription).finish();

    // Route GET requests to the playground and POST requests to the custom gql_handler
    let app = Router::new()
        .route("/graphql", get(playground).post(gql_handler))
        .with_state(schema); // Inject the schema as application state

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Running on http://localhost:3000/graphql");

    axum::serve(listener, app).await.unwrap();
}
