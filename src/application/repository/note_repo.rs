use chrono::Utc;
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    application::{repository::RepositoryResult, state::SharedState},
    domain::models::note::Note,
};

pub async fn list(state: &SharedState) -> RepositoryResult<Vec<Note>> {
    let users = query_as::<_, Note>("SELECT * FROM notes")
        .fetch_all(&state.db_pool)
        .await?;

    Ok(users)
}

pub async fn list_by_user(user_id: Uuid, state: &SharedState) -> RepositoryResult<Vec<Note>> {
    let users = query_as::<_, Note>("SELECT * FROM notes WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(&state.db_pool)
        .await?;

    Ok(users)
}

pub async fn get_by_id(id: Uuid, state: &SharedState) -> RepositoryResult<Note> {
    let user = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await?;
    Ok(user)
}

pub async fn add(note: Note, state: &SharedState) -> RepositoryResult<Note> {
    let time_now = Utc::now().naive_utc();
    tracing::trace!("note: {:#?}", note);
    let note = sqlx::query_as::<_, Note>(
        r#"INSERT INTO notes (id,
         user_id,
         text,
         created_at,
         updated_at)
         VALUES ($1,$2,$3,$4,$5)
         RETURNING notes.*"#,
    )
    .bind(note.id)
    .bind(note.user_id)
    .bind(note.text)
    .bind(time_now)
    .bind(time_now)
    .fetch_one(&state.db_pool)
    .await?;

    Ok(note)
}

pub async fn update(note: Note, state: &SharedState) -> RepositoryResult<Note> {
    tracing::trace!("note: {:#?}", note);
    let time_now = Utc::now().naive_utc();
    let note = sqlx::query_as::<_, Note>(
        r#"UPDATE notes
         SET 
         user_id = $1,
         text = $2,
         updated_at = $3
         WHERE id = $4
         RETURNING notes.*"#,
    )
    .bind(note.user_id)
    .bind(note.text)
    .bind(time_now)
    .bind(note.id)
    .fetch_one(&state.db_pool)
    .await?;

    Ok(note)
}

pub async fn delete(id: Uuid, state: &SharedState) -> RepositoryResult<bool> {
    let query_result = sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await?;

    Ok(query_result.rows_affected() == 1)
}
