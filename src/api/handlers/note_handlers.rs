use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use sqlx::types::Uuid;
use thiserror::Error;

use crate::{
    api::{
        APIError, APIErrorCode, APIErrorEntry, APIErrorKind,
        error::API_DOCUMENT_URL,
        version::{self, APIVersion},
    },
    application::{
        repository::note_repo,
        security::jwt::{AccessClaims, ClaimsMethods},
        state::SharedState,
    },
    domain::models::note::Note,
    domain::models::user::SimpleUser,
};

pub async fn list_notes_handler(
    api_version: APIVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
) -> Result<Json<Vec<Note>>, APIError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    access_claims.validate_role_admin()?;
    let notes = note_repo::list(&state).await?;
    Ok(Json(notes))
}

pub async fn list_notes_by_user_handler(
    api_version: APIVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
    Json(user): Json<SimpleUser>,
) -> Result<Json<Vec<Note>>, APIError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    access_claims.validate_role_admin()?;
    let notes = note_repo::list_by_user(user.id, &state).await?;
    Ok(Json(notes))
}

pub async fn get_note_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<Json<Note>, APIError> {
    let api_version: APIVersion = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    let user = note_repo::get_by_id(id, &state)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                let user_error = NoteError::NoteNotFound(id);
                (user_error.status_code(), APIErrorEntry::from(user_error)).into()
            }
            _ => APIError::from(e),
        })?;

    Ok(Json(user))
}

pub async fn add_note_handler(
    api_version: APIVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
    Json(mut note): Json<Note>,
) -> Result<impl IntoResponse, APIError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    access_claims.validate_role_admin()?;
    let naive_now = Utc::now().naive_utc();
    note.created_at = Some(naive_now);
    note.updated_at = Some(naive_now);
    let note = note_repo::add(note, &state).await?;
    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn update_note_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
    Json(note): Json<Note>,
) -> Result<Json<Note>, APIError> {
    let api_version: APIVersion = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    let note = note_repo::update(note, &state).await?;
    Ok(Json(note))
}

pub async fn delete_note_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, APIError> {
    let api_version: APIVersion = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    if note_repo::delete(id, &state).await? {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

#[derive(Debug, Error)]
enum NoteError {
    #[error("note not found: {0}")]
    NoteNotFound(Uuid),
}

impl NoteError {
    const fn status_code(&self) -> StatusCode {
        match self {
            Self::NoteNotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<NoteError> for APIErrorEntry {
    fn from(note_error: NoteError) -> Self {
        let message = note_error.to_string();
        match note_error {
            NoteError::NoteNotFound(note_id) => Self::new(&message)
                .code(APIErrorCode::UserNotFound)
                .kind(APIErrorKind::ResourceNotFound)
                .description(&format!("note with the ID '{}' does not exist in our records", note_id))
                .detail(serde_json::json!({"note_id": note_id}))
                .reason("must be an existing user")
                .instance(&format!("/api/v1/notes/{}", note_id))
                .trace_id()
                .help(&format!("please check if the user ID is correct or refer to our documentation at {}#errors for more information", API_DOCUMENT_URL))
                .doc_url(),
        }
    }
}
