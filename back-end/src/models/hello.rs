use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use utoipa::{ToSchema, schema}; 


#[derive(ToSchema, MultipartForm)]
pub struct HelloForm {
    #[multipart(limit = "10mb")]
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    pub file: TempFile,
    #[schema(value_type = String)]
    pub name: Text<String>,
}
