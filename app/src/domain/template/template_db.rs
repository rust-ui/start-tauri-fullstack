use serde::{Deserialize, Serialize};
use struct_field_names::StructFieldNames;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, StructFieldNames)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Template {
    pub unid: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[cfg(feature = "ssr")]
mod db {
    use sqlx::{PgPool, query_as};

    use super::*;

    impl Template {
        pub async fn get_all(pool: &PgPool) -> Result<Vec<Template>, sqlx::Error> {
            query_as!(
                Template,
                r#"
                    SELECT
                        unid,
                        title,
                        description,
                        created_at,
                        updated_at
                    FROM templates
                "#
            )
            .fetch_all(pool)
            .await
        }

        pub async fn get_by_unid(pool: &PgPool, unid: Uuid) -> Result<Option<Template>, sqlx::Error> {
            query_as!(
                Template,
                r#"
                    SELECT
                        unid,
                        title,
                        description,
                        created_at,
                        updated_at
                    FROM templates
                    WHERE unid = $1
                "#,
                unid
            )
            .fetch_optional(pool)
            .await
        }

        pub async fn add(pool: &PgPool, title: String, description: String) -> Result<Template, sqlx::Error> {
            query_as!(
                Template,
                r#"
                    INSERT INTO templates (title, description)
                    VALUES ($1, $2)
                    RETURNING unid, title, description, created_at, updated_at
                "#,
                title,
                description
            )
            .fetch_one(pool)
            .await
        }

        pub async fn delete(pool: &PgPool, unid: Uuid) -> Result<Uuid, sqlx::Error> {
            let result = sqlx::query!(
                r#"
                    DELETE FROM templates
                    WHERE unid = $1
                    RETURNING unid
                "#,
                unid
            )
            .fetch_one(pool)
            .await?;

            Ok(result.unid)
        }
    }
}
