use actix_web::{post};
use actix_web::{web, HttpResponse};
use crate::DbPool;
use crate::model::site_model::NewSite;
use crate::repository::site_repository;
use crate::routes::workers::page_submit::GenericOK;
use crate::{AppResult};

pub fn query_config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_site);
}

// new_site: web::Json<Vec<NewSite>>

#[post("/workers/site/submit")]
pub async fn submit_site(db: web::Data<DbPool>, json: web::Json<Vec<NewSite>>) -> AppResult<HttpResponse> {

    // print post data

    let mut tx = db.begin().await.unwrap();
    for site in json.iter() {
        let site = site.clone();
        site_repository::insert_site(&mut tx, site).await.unwrap();
    }
    tx.commit().await.ok();

    let response: GenericOK = GenericOK {
        success: true
    };

    Ok(HttpResponse::Ok().json(response))

}