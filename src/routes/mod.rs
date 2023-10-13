pub mod user;

use axum::Router;

use user::user_router;

pub fn routes() -> Router {
    Router::new().nest("/user", user_router())
}
