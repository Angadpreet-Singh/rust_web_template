use axum::Router;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

mod controllers;
mod middlewares;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().merge(routes::routes());

    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap_or(3000),
        Err(_) => 3000,
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("--> Server is listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
