use crate::{
    model::{site_model::{NewSite}},
    Tx, infra::error::DbError
};

/// Insert a new site into the site table
pub async fn insert_site(tx: &mut Tx, new_site: NewSite) -> Result<(), DbError> {
    let query = sqlx::query!(
        r#"
        INSERT INTO public."site" (name, url)
        VALUES ($1, $2)
        "#,
        new_site.name,
        new_site.url
    );

    println!("Inserting site: {}", new_site.url);

    query.execute(tx).await;

    Ok(())
}