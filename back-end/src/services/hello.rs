use crate::models::hello::HelloForm;
use actix_multipart::form::MultipartForm;
use actix_web::{post, Responder};

#[utoipa::path(
    request_body(content = HelloForm, content_type = "multipart/form-data")
)]
#[post("/hello")]
async fn hello_form(MultipartForm(form): MultipartForm<HelloForm>) -> impl Responder {
    let name = form.name.to_string();
    let file = &form.file;
    format!(
        "Greetings: name: {name}, type: {} size: {} file_name: {}!",
        file.content_type
            .as_ref()
            .map(|mime| mime.to_string())
            .unwrap_or_default(),
        file.size,
        file.file_name.as_ref().unwrap_or(&String::new())
    )
}