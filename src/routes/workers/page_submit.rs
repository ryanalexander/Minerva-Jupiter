#[actix_web::post("/workers/page/submit")]
pub async fn submit_page(db: Data<DbPool>, url: str) -> Result<HttpResponse, AppError> {
    
}