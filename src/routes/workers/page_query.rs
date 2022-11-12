use actix_web::{web, HttpResponse};

use crate::DbPool;
use crate::{repository::link_repository, AppResult};

pub fn query_config(cfg: &mut web::ServiceConfig) {
    cfg.service(request_page);
}

// TODO - Streamline this, we should be able to receive either a site or nothing after await
#[actix_web::get("/workers/scrape/job")]
pub async fn request_page(db: web::Data<DbPool>) -> AppResult<HttpResponse> {
    let mut tx = db.begin().await.unwrap();
    let site = link_repository::fetch_unscraped_link(&mut tx)
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(site))
}
