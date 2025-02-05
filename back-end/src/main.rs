use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use std::env;
use std::io;
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
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()); // Lendo porta do ambiente
    let port: u16 = port.parse().expect("PORT inválida");

    info!("Servidor iniciando na porta {}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        http::header::CONTENT_TYPE,
                        http::header::AUTHORIZATION,
                    ])
                    .max_age(3600),
            )
            .service(get_block)
            .service(get_transaction)
            .service(get_balance)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
