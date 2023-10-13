use crate::controllers::user;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn user_router() -> Router {
    Router::new()
        .route("/", post(user::create_user))
        .route("/", get(user::get_all_user))
        .route("/:id", get(user::get_user_by_id))
        .route("/:id", delete(user::delete_user_by_id))
        .route("/:id", put(user::update_user_by_id))
}
