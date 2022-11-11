use crate::{infra::error::DbError, model::link_model::Link, Tx};

/// Insert a new link into the link table
pub async fn insert_link(tx: &mut Tx, link: String) {
    let query = sqlx::query!(
        r#"
        INSERT INTO public."Link" (url)
        VALUES ($1)
        "#,
        link
    );
    query.execute(tx).await;
}

/// Fetch a link that has not been scraped within the last day
pub async fn fetch_unscraped_link(tx: &mut Tx) -> Result<Link, DbError> {
    sqlx::query_as!(
        Link,
        r#"SELECT * FROM public."Link" WHERE scraped = false OR CURRENT_TIMESTAMP - "lastScraped" > '1 day'::interval ORDER BY scraped LIMIT 1;"#,
    ).fetch_one(tx).await.map_err(DbError::from)
}
