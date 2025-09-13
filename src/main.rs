use axum::Router;
use env_logger;
use server::{config, route::route, server::Server};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    env_logger::init();

    let cfg = config::from_env();

    let router = Router::new();
    let router = route(router);

    let app = Server {
        config: cfg,
        router,
    };

    if let Err(err) = app.run().await {
        eprintln!("server failed: {err}");
        std::process::exit(1);
    }
}
