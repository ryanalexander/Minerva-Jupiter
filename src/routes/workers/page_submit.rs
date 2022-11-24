use actix_web::{web, HttpRequest, HttpResponse};
use serde::Serialize;
use crate::DbPool;

use crate::AppResult;
use crate::model::link_model::NewLink;
use crate::repository::link_repository;

pub fn query_config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_page);
}

#[actix_web::post("/workers/scrape/submit")]
pub async fn submit_page(db: web::Data<DbPool>, json: web::Json<Vec<NewLink>>) -> AppResult<HttpResponse> {

    let mut tx = db.begin().await.unwrap();
    for link in json.iter() {
        let link = link.clone();
        link_repository::insert_link(&mut tx, link.url).await.unwrap();
    }
    tx.commit().await.ok();

    let response: GenericOK = GenericOK {
        success: true
    };

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Serialize)]
pub struct GenericOK {
    pub success: bool,
}
