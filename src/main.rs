// src/main.rs
use actix_web::{web, App, HttpServer};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use social_network_analysis::graphql::{QueryRoot, SocialNetworkSchema};
use social_network_analysis::models::User;

mod graphql;
mod models;

async fn index(schema: web::Data<SocialNetworkSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = SocialNetworkSchema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
