use chrono::Utc;
use uuid::Uuid;

use crate::{
    application::{repository::RepositoryResult, state::SharedState},
    domain::models::stats::{StatRequest, StatResponse},
};

pub async fn get_by_id(id: Uuid, state: &SharedState) -> RepositoryResult<StatResponse> {
    let user = sqlx::query_as::<_, StatResponse>("SELECT * FROM stats WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await?;
    Ok(user)
}

pub async fn update(stat: StatRequest, state: &SharedState) -> RepositoryResult<StatResponse> {
    tracing::trace!("stat: {:#?}", stat);
    let time_now = Utc::now().naive_utc();

    let stat_rep = sqlx::query_as::<_, StatResponse>("SELECT * FROM stats WHERE id = $1")
        .bind(stat.user_id)
        .fetch_all(&state.db_pool)
        .await?;

    if stat_rep.len() == 0 {
        let stat_id = Uuid::new_v4();
        let stat: StatResponse = sqlx::query_as::<_, StatResponse>(
            r#"INSERT INTO stats (id,
             user_id,
             notes,
             created_at,
             updated_at)
             VALUES ($1,$2,$3,$4,$5)
             RETURNING stats.*"#,
        )
        .bind(stat_id)
        .bind(stat.user_id)
        .bind(stat.value)
        .bind(time_now)
        .bind(time_now)
        .fetch_one(&state.db_pool)
        .await?;

        Ok(stat)
    } else {
        if stat.value < 0 {
            let stat = sqlx::query_as::<_, StatResponse>(
                r#"UPDATE stats
                SET 
                notes = notes - 1,
                updated_at = $1
                WHERE user_id = $2
                RETURNING stats.*"#,
            )
            .bind(time_now)
            .bind(stat.user_id)
            .fetch_one(&state.db_pool)
            .await?;
            Ok(stat)
        } else {
            let stat = sqlx::query_as::<_, StatResponse>(
                r#"UPDATE stats
                SET 
                notes = notes + 1,
                updated_at = $1
                WHERE user_id = $2
                RETURNING stats.*"#,
            )
            .bind(time_now)
            .bind(stat.user_id)
            .fetch_one(&state.db_pool)
            .await?;
            Ok(stat)
        }
    }
}
