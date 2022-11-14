use actix_web::{web, HttpResponse};

use crate::DbPool;
use crate::{repository::link_repository, AppResult};

pub fn query_config(cfg: &mut web::ServiceConfig) {
    cfg.service(request_page);
}

#[actix_web::get("/workers/site/job")]
pub async fn request_page(db: web::Data<DbPool>) -> AppResult<HttpResponse> {
    let mut tx = db.begin().await.unwrap();
    let site = link_repository::fetch_siteless_link(&mut tx, 32)
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(site))
}
