use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    api::handlers::stats_handlers::{get_stats_handler, update_stats_count},
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/{id}", get(get_stats_handler))
        .route("/user", post(update_stats_count))
}
