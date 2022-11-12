use actix_web::post;
use actix_web::{web, HttpResponse};
use crate::DbPool;
use crate::model::site_model::NewSite;
use crate::repository::site_repository;
use crate::routes::workers::page_submit::GenericOK;
use crate::{AppResult};

pub fn query_config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_site);
}

#[post("/workers/site/submit")]
pub async fn submit_site(db: web::Data<DbPool>, new_site: web::Json<NewSite>) -> AppResult<HttpResponse> {
    let mut tx = db.begin().await.unwrap();
    let site = new_site.into_inner();
    site_repository::insert_site(&mut tx, site).await;
    tx.commit().await;

    let response: GenericOK = GenericOK {
        success: true
    };

    Ok(HttpResponse::Ok().json(response))

}