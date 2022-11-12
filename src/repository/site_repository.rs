use sqlx::postgres::PgQueryResult;

use crate::{
    model::{site_model::{NewSite}}, Tx,
};

/// Insert a new site into the site table
pub async fn insert_site(tx: &mut Tx, new_site: NewSite) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO public."site" (name, url)
        VALUES ($1, $2)
        "#,
        new_site.name,
        new_site.url
    );

    println!("Inserting site: {}", new_site.url);

    return query.execute(tx).await;
}