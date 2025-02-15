use actix_web::{get,post, web, Responder, HttpResponse};
use reqwest::Client;
use std::env;
use log::{error, warn};
use serde::{Serialize, Deserialize};
use stellar_sdk::Keypair;
use aes_gcm::{Aes128Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::Rng;
use base64::{engine::general_purpose, Engine as _};
use utoipa::ToSchema;


use crate::models::stellar::{Account, Ledger, Transaction};

async fn get_base_url() -> String {
    env::var("RPC_URL").unwrap_or_else(|_| "http://34.60.10.29:8000".to_string())
}

const AES_KEY: [u8; 16] = *b"0123456789abcdef"; // Correção: Agora é um array fixo, sem referência

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct KeyPairResponse {
    public_key: String,
    encrypted_private_key: String,
}

#[utoipa::path(
    post,
    path = "/generate_keys",
    responses((status = 200, description = "Par de chaves gerado com sucesso", body = KeyPairResponse))
)]
#[post("/generate_keys")]
async fn generate_keys() -> impl Responder {
    let mut keypair = Keypair::random().expect("Erro ao gerar chave"); // 🔥 Adicionado `mut`
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
    let encrypted_data = cipher.encrypt(nonce, private_key.as_bytes()).expect("Erro na criptografia");

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
    responses((status = 200, description = "Transação retornada com sucesso", body = Transaction))
)]
#[get("/transaction/{hash}")]
async fn get_transaction(hash: web::Path<String>) -> impl Responder {
    fetch_data::<Transaction>(&format!("transactions/{}", hash.into_inner())).await
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
async fn fetch_data<T: serde::de::DeserializeOwned + serde::Serialize>(endpoint: &str) -> impl Responder {
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
                HttpResponse::InternalServerError().body("Erro inesperado ao acessar a API da Stellar.")
            }
        }
    }
}

