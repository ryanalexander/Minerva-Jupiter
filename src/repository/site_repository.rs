use sqlx::postgres::PgQueryResult;

use crate::{
    model::{site_model::{NewSite}}, Tx,
};

/// Insert a new site into the site table
pub async fn insert_site(tx: &mut Tx, new_site: NewSite) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO site (name, url) SELECT * FROM (VALUES ($1, $2)) AS i(name, url) WHERE NOT EXISTS (SELECT hash FROM site_blacklist WHERE hash = md5(i.url)) ON CONFLICT DO NOTHING
        "#,
        new_site.name,
        new_site.url
    );

    println!("Inserting site: {}", new_site.url);

    return query.execute(tx).await;
}

pub async fn domain_blacklisted(tx: &mut Tx, domain: String) -> bool {
    let query = sqlx::query!("SELECT hash FROM site_blacklist WHERE hash = md5($1)", domain).fetch_one(tx).await;

    return query.is_ok();
}