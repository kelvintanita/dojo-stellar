use std::io;

use actix_web::{App, HttpServer};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod services;

use crate::services::hello::hello_form;
use crate::services::user::get_user;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .into_utoipa_app()
            .service(hello_form)
            .service(get_user)
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", api)
            })
            .into_app()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
