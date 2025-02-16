use utoipa::OpenApi;
use crate::services::stellar::__path_get_balance;
use crate::services::stellar::__path_get_block;
use crate::services::stellar::__path_get_transaction;
use crate::services::stellar::__path_generate_keys;
use crate::services::stellar::__path_send_xlm;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Dojo Stellar API",
        description = "API para interagir com a blockchain Stellar.",
        contact(
            name = "Kelvin Tanita",
            email = "kelvintamota@gmail.com",
            url = "https://github.com/kelvintanita"
        ),
        license(name = "MIT", url = "https://opensource.org/licenses/MIT"),
        version = "1.0.0"
    ),
    paths(get_balance, get_block, get_transaction, generate_keys, send_xlm)
)]
pub struct ApiDoc;