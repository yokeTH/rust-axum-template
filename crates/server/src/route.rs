use axum::{Router, routing::get};

pub fn route(r: Router) -> Router {
    r.route("/", get(hello))
}

async fn hello() -> &'static str {
    "Hello Na"
}
