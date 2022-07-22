use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use std::net::SocketAddr;

pub mod data;
pub mod graphql;

use graphql::GqlSchema;

async fn graphql_handler(schema: Extension<GqlSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/ws"),
    ))
}

#[tokio::main]
async fn main() {
    let gql_schema = graphql::schema();

    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/ws", GraphQLSubscription::new(gql_schema.clone()))
        .layer(Extension(gql_schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
