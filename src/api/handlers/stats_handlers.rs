use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::types::Uuid;
use thiserror::Error;

use crate::{
    api::{
        APIError, APIErrorCode, APIErrorEntry, APIErrorKind,
        error::API_DOCUMENT_URL,
        version::{self, APIVersion},
    },
    application::{
        repository::stats_repo,
        security::jwt::{AccessClaims, ClaimsMethods},
        state::SharedState,
    },
    domain::models::stats::{StatRequest, StatResponse},
};

pub async fn get_stats_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<Json<StatResponse>, APIError> {
    let api_version: APIVersion = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    let stats = stats_repo::get_by_id(id, &state)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                let stats_error = StatsError::StatsNotFound(id);
                (stats_error.status_code(), APIErrorEntry::from(stats_error)).into()
            }
            _ => APIError::from(e),
        })?;

    Ok(Json(stats))
}

pub async fn update_stats_count(
    access_claims: AccessClaims,
    State(state): State<SharedState>,
    Json(stat): Json<StatRequest>,
) -> Result<impl IntoResponse, APIError> {
    tracing::trace!("authentication details: {:#?}", access_claims);
    access_claims.validate_role_admin()?;
    let stat = stats_repo::update(stat, &state).await?;
    Ok((StatusCode::CREATED, Json(stat)))
}

#[derive(Debug, Error)]
enum StatsError {
    #[error("stats not found: {0}")]
    StatsNotFound(Uuid),
}

impl StatsError {
    const fn status_code(&self) -> StatusCode {
        match self {
            Self::StatsNotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<StatsError> for APIErrorEntry {
    fn from(stats_error: StatsError) -> Self {
        let message = stats_error.to_string();
        match stats_error {
            StatsError::StatsNotFound(user_id) => Self::new(&message)
                .code(APIErrorCode::UserNotFound)
                .kind(APIErrorKind::ResourceNotFound)
                .description(&format!("stats for user with the ID '{}' does not exist in our records", user_id))
                .detail(serde_json::json!({"user_id": user_id}))
                .reason("must be an existing user")
                .instance(&format!("/api/v1/stats/{}", user_id))
                .trace_id()
                .help(&format!("please check if the user ID is correct or refer to our documentation at {}#errors for more information", API_DOCUMENT_URL))
                .doc_url(),
        }
    }
}
