use sqlx::postgres::PgQueryResult;

use crate::{infra::error::DbError, model::link_model::Link, Tx};

/// Insert a new link into the link table
pub async fn insert_link(tx: &mut Tx, link: String) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO public."link" (url)
        VALUES ($1)
        "#,
        link
    );
    return query.execute(tx).await;
}

/// Fetch a link that has not been scraped within the last day
pub async fn fetch_unscraped_link(tx: &mut Tx) -> Result<Link, DbError> {
    sqlx::query_as!(
        Link,
        r#"SELECT * FROM public."link" WHERE scraped = false OR CURRENT_TIMESTAMP - "last_scraped" > '1 day'::interval ORDER BY scraped LIMIT 1;"#,
    ).fetch_one(tx).await.map_err(DbError::from)
}

pub async fn fetch_siteless_link(tx: &mut Tx) -> Result<Link, DbError> {
    sqlx::query_as!(
        Link,
        r#"SELECT * FROM public."link" WHERE "site_id" IS NULL ORDER BY scraped LIMIT 1;"#,
    ).fetch_one(tx).await.map_err(DbError::from)
}
