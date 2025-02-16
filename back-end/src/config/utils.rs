use std::env;

/// Função utilitária para assinar um XDR e converter para Base64
// pub fn sign_transaction_xdr(keypair: &Keypair, unsigned_xdr: &str) -> Result<String, String> {
//     match keypair.sign(unsigned_xdr.as_bytes()) {
//         Ok(signature) => {
//             let signed_xdr = general_purpose::STANDARD.encode(&signature);
//             Ok(signed_xdr)
//         },
//         Err(e) => {
//             error!("Erro ao assinar a transação: {}", e);
//             Err("Erro ao assinar a transação".to_string())
//         }
//     }
// }

/// Função utilitária assíncrona para obter a URL base do RPC
pub async fn get_base_url() -> String {
    env::var("RPC_URL").unwrap_or_else(|_| "http://34.60.10.29:8000".to_string())
}
