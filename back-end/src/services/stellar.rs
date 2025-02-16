use crate::config::constants::{AES_KEY, HORIZON_URL};
use crate::models::stellar::{Account, KeyPairResponse, Ledger, TransactionModel, SendXlmRequest};
use crate::config::utils::{ get_base_url };
use actix_web::{get, post, web, HttpResponse, Responder};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Key, Nonce};
use base64::{engine::general_purpose, Engine as _};
use log::{error, info, warn};
use rand::Rng;
use reqwest::Client;
use serde_json;
use stellar_sdk::Keypair;


#[utoipa::path(
    post,
    path = "/send_xlm",
    request_body = SendXlmRequest,
    responses(
        (status = 200, description = "Transação enviada com sucesso", body = String),
        (status = 400, description = "Chave secreta inválida"),
        (status = 500, description = "Erro ao processar a transação")
    )
)]
#[post("/send_xlm")]
async fn send_xlm(req: web::Json<SendXlmRequest>) -> impl Responder {
    let client = Client::new();

    // Criando a chave do remetente
    let sender_keypair = match Keypair::from_secret_key(&req.sender_secret) {
        Ok(kp) => kp,
        Err(_) => return HttpResponse::BadRequest().body("Chave secreta inválida"),
    };

    // Criando a requisição para o Horizon criar a transação XDR
    let transaction_request = serde_json::json!({
        "source": sender_keypair.public_key(),
        "operations": [{
            "type": "payment",
            "destination": req.recipient,
            "asset": {
                "type": "native"  // XLM
            },
            "amount": req.amount
        }]
    });

    let horizon_url = format!("{}/transactions", HORIZON_URL);
    let transaction_response = match client.post(&horizon_url).json(&transaction_request).send().await {
        Ok(res) => res,
        Err(err) => {
            error!("Erro ao criar transação no Horizon: {}", err);
            return HttpResponse::InternalServerError().body("Erro ao criar transação");
        }
    };

    let transaction_data: serde_json::Value = match transaction_response.json().await {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Erro ao processar resposta do Horizon");
        }
    };

    // Assinando o XDR gerado pelo Horizon
    let unsigned_xdr = transaction_data["transaction"].as_str().unwrap_or("");
    let signature = match sender_keypair.sign(unsigned_xdr.as_bytes()) {
        Ok(sig) => sig,
        Err(e) => {
            error!("Erro ao assinar a transação: {}", e);
            return HttpResponse::InternalServerError().body("Erro ao assinar a transação");
        }
    };
    
    // Converte o `Vec<u8>` assinado para Base64
    let signed_xdr = general_purpose::STANDARD.encode(&signature);

    // Enviando a transação assinada de volta para o Horizon
    let submit_request = serde_json::json!({
        "tx": signed_xdr
    });

    let send_response = match client.post(&horizon_url).json(&submit_request).send().await {
        Ok(res) => res,
        Err(err) => {
            error!("Erro ao enviar transação para o Horizon: {}", err);
            return HttpResponse::InternalServerError().body("Erro ao enviar transação");
        }
    };

    let response_text = match send_response.text().await {
        Ok(text) => text,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Erro ao processar resposta do envio");
        }
    };

    info!("Transação enviada com sucesso: {}", response_text);
    HttpResponse::Ok().body(response_text)
}

#[utoipa::path(
    post,
    path = "/generate_keys",
    responses((status = 200, description = "Par de chaves gerado com sucesso", body = KeyPairResponse))
)]
#[post("/generate_keys")]
async fn generate_keys() -> impl Responder {
    let mut keypair = Keypair::random().expect("Erro ao gerar chave"); 
    let public_key = keypair.public_key();
    let private_key = keypair.secret_key().expect("Erro ao obter chave privada");

    // Gerando nonce aleatório (12 bytes)
    let mut rng = rand::thread_rng();
    let nonce_bytes: [u8; 12] = rng.gen();
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 🔥 Correção: Passar `AES_KEY` corretamente como slice (`&AES_KEY`)
    let key = Key::<Aes128Gcm>::from_slice(&AES_KEY);
    let cipher = Aes128Gcm::new(key);

    // Criptografando a chave privada com AES-128-GCM
    let encrypted_data = cipher
        .encrypt(nonce, private_key.as_bytes())
        .expect("Erro na criptografia");

    let encrypted_private_key = format!(
        "{}:{}",
        general_purpose::STANDARD.encode(&nonce_bytes),
        general_purpose::STANDARD.encode(&encrypted_data)
    );

    HttpResponse::Ok().json(KeyPairResponse {
        public_key,
        encrypted_private_key,
    })
}

/// Buscar um bloco pelo número
#[utoipa::path(
    get,
    path = "/block/{sequence}",
    params(("sequence" = u32, Path, description = "Número do bloco")),
    responses((status = 200, description = "Bloco retornado com sucesso", body = Ledger))
)]
#[get("/block/{sequence}")]
async fn get_block(sequence: web::Path<u32>) -> impl Responder {
    fetch_data::<Ledger>(&format!("ledgers/{}", sequence.into_inner())).await
}

/// Buscar uma transação pelo hash
#[utoipa::path(
    get,
    path = "/transaction/{hash}",
    params(("hash" = String, Path, description = "Hash da transação")),
    responses((status = 200, description = "Transação retornada com sucesso", body = TransactionModel))
)]
#[get("/transaction/{hash}")]
async fn get_transaction(hash: web::Path<String>) -> impl Responder {
    fetch_data::<TransactionModel>(&format!("transactions/{}", hash.into_inner())).await
}

/// Buscar o saldo pelo endereço
#[utoipa::path(
    get,
    path = "/balance/{account_id}",
    params(("account_id" = String, Path, description = "Endereço da conta Stellar")),
    responses((status = 200, description = "Saldo retornado com sucesso", body = Vec<Account>))
)]
#[get("/balance/{account_id}")]
async fn get_balance(account_id: web::Path<String>) -> impl Responder {
    fetch_data::<Account>(&format!("accounts/{}", account_id.into_inner())).await
}

/// Função genérica para buscar dados da API
async fn fetch_data<T: serde::de::DeserializeOwned + serde::Serialize>(
    endpoint: &str,
) -> impl Responder {
    let client = Client::new();
    let base_url = get_base_url().await;
    let url = format!("{}/{}", base_url, endpoint);

    match client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();

            if status == reqwest::StatusCode::NOT_FOUND {
                warn!("Recurso não encontrado: {}", url);
                return HttpResponse::NotFound().body("Recurso não encontrado.");
            }

            if status.is_success() {
                match response.json::<T>().await {
                    Ok(data) => HttpResponse::Ok().json(data),
                    Err(err) => {
                        error!("Erro ao deserializar a resposta de {}: {}", url, err);
                        HttpResponse::InternalServerError().body("Erro ao deserializar a resposta.")
                    }
                }
            } else {
                error!("Erro na resposta da API para {}: {:?}", url, status);
                HttpResponse::InternalServerError().body("Erro ao acessar a API da Stellar.")
            }
        }
        Err(err) => {
            error!("Erro ao conectar-se à API: {}", err);
            if err.is_connect() {
                HttpResponse::ServiceUnavailable().body("Erro de conexão com a API da Stellar.")
            } else {
                HttpResponse::InternalServerError()
                    .body("Erro inesperado ao acessar a API da Stellar.")
            }
        }
    }
}
