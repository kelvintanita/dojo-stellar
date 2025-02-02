use std::io;

use actix_web::{App, HttpServer};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use dotenv::dotenv;
use log::info;

mod models;
mod services;

use crate::services::stellar::{get_account, get_account_transactions, get_ledger, get_offer};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    info!("========= Servidor iniciado na porta 8080 =========");
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .into_utoipa_app()
            .service(get_account)
            .service(get_account_transactions)
            .service(get_offer)
            .service(get_ledger)
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", api)
            })
            .into_app()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
