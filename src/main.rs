mod infra;

use std::time::Duration;

use dotenv::dotenv;
use sqlx::{
    pool::PoolOptions,
    postgres::{PgConnectOptions, PgSslMode},
    Pool, Postgres,
};
use Jupiter::run_webserver;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    /*
     * Database connections and checks
     */
    let db_options = PgConnectOptions::default()
        .application_name("Minerva")
        .host(
            &std::env::var("POSTGRES_HOST")
                .expect("No database host in environment. Use ./setup to run through setup!"),
        )
        .username(
            &std::env::var("POSTGRES_USER")
                .expect("No database user in environment. Use ./setup to run through setup!"),
        )
        .password(
            &std::env::var("POSTGRES_PASS")
                .expect("No database password in environment. Use ./setup to run through setup!"),
        )
        .database(
            &std::env::var("POSTGRES_DB")
                .expect("No database name in environment. Use ./setup to run through setup!"),
        )
        .ssl_mode(PgSslMode::Prefer);

    let db_pool: Pool<Postgres> = PoolOptions::default()
        .acquire_timeout(Duration::from_secs(5))
        .connect_lazy_with(db_options);

    /*
     * Webserver start and checks
     */

    let http_addr = format!(
        "{}:{}",
        std::env::var("HTTP_HOST")
            .expect("No HTTP host in environment. Use ./setup to run through setup!"),
        std::env::var("HTTP_PORT")
            .expect("No HTTP port in environment. Use ./setup to run through setup!")
    );

    let http_listener =
        std::net::TcpListener::bind(http_addr).expect("Failed to bind HTTP listener");

    // Yes this is uguly, but it works
    let _server = run_webserver(http_listener, db_pool.clone())
        .await
        .unwrap()
        .await;

    Ok(())
}
