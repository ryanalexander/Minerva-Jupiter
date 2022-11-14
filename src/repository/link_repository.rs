use sqlx::postgres::PgQueryResult;

use crate::{infra::error::DbError, model::link_model::{Link, Domain}, Tx};

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

pub async fn fetch_siteless_link(tx: &mut Tx, count: i64) -> Result<Vec<Domain>, DbError> {
    sqlx::query_as!(
        Domain,
        r#"SELECT DISTINCT domain FROM link l1 LEFT JOIN site s1 ON s1.url = l1.domain WHERE s1.url IS NULL LIMIT $1;"#,
        count
    ).fetch_all(tx).await.map_err(DbError::from)
}
