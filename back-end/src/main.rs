use std::io;

use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use dotenv::dotenv;
use log::info;

mod config;
mod models;
mod services;

use crate::config::swagger::ApiDoc;
use crate::services::stellar::{get_balance, get_block, get_transaction};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    info!("========= Servidor iniciado na porta 8080 | http://127.0.0.1:8080 =========");
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(get_block)
            .service(get_transaction)
            .service(get_balance)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", ApiDoc::openapi()), 
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
