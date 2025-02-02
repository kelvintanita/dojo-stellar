use actix_web::{get, web, Responder};
use crate::models::user::User;

#[utoipa::path(
    get,
    path = "/user/{id}",
    params(
        ("id" = u32, Path, description = "ID do usuário")
    ),
    responses(
        (status = 200, description = "Usuário retornado com sucesso", body = User)
    )
)]
#[get("/user/{id}")]
async fn get_user(id: web::Path<u32>) -> impl Responder {
    let user = User {
        id: id.into_inner(),
        name: "Kelvin".to_string(),
    };
    web::Json(user)
}
