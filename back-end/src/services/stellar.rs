use actix_web::{get, web, Responder, HttpResponse};
use reqwest::Client;
use std::env;
use log::{error, warn};

use crate::models::stellar::{Account, Ledger, Transaction};

async fn get_base_url() -> String {
    env::var("RPC_URL").unwrap_or_else(|_| "http://34.60.10.29:8000".to_string())
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

