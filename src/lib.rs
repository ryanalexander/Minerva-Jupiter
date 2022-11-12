//! Start all services required for the node.
#![feature(const_option_ext)]

use std::{net::TcpListener};
use infra::{logging, error::AppError};
use actix_web::{HttpServer, App, HttpResponse, HttpRequest, http::header::ContentType, dev::Server};
use sqlx::{PgPool, Transaction, Postgres};
use actix_web::web;

pub mod repository;
pub mod model;
pub mod infra;
pub mod routes;

pub type DbPool = PgPool;

pub type AppResult<T> = Result<T, AppError>;
pub type Tx = Transaction<'static, Postgres>;

pub const Version:&str = option_env!("GIT_HASH").unwrap_or("unknown");

// Test page -- To be removed
async fn test(_req: HttpRequest) -> HttpResponse { 
    let web_response = "<b>Warning:</b> mysqli::mysqli(): (HY000/1049): You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near ''' in <b>A:\\NotPorn\\servers\\backend\\htdocs\\lib\\sql\\sql.php</b> on line <b>69</b>";
    if _req.query_string().contains('q') {
        HttpResponse::Ok().content_type(ContentType::html()).body("uh oh, you found my backdoor... Please send help, they don't feed me!")
    } else {
        HttpResponse::InternalServerError().content_type(ContentType::html()).body(web_response)
    }
}

// Start a WebServer
pub async fn run_webserver(http_listener: TcpListener, db_pool: DbPool) -> anyhow::Result<Server> {
    logging::log_info(&format!("Starting Jupiter webserver build: {}", Version));

    let pool = web::Data::new(db_pool);
    let listener = http_listener.try_clone().expect("Failed to clone HTTP listener");

    let server = HttpServer::new(move || {
        App::new()
            // Database 
            .app_data(pool.clone())

            // Static paths
            .route("/test", web::get().to(test))

            // Dynamic paths
            .configure(routes::workers::page_query::query_config)
            .configure(routes::workers::page_submit::query_config)
            .configure(routes::workers::site_submit::query_config)
    }).listen(http_listener)?.run();

    logging::log_success(&format!("Jupiter server now listening on port {}", listener.local_addr().unwrap().port()));

    Ok(server)
}