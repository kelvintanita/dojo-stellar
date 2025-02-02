use actix_web::{get, web, Responder, HttpResponse};
use reqwest::Client;
use std::env;
use crate::models::stellar::{Account, Transaction, Offer, Ledger};

async fn get_base_url() -> String {
    env::var("RPC_URL").unwrap_or_else(|_| "http://34.60.10.29:8000".to_string())
}

/// Obtém os detalhes de uma conta na Stellar
#[utoipa::path(
    get,
    path = "/accounts/{account_id}",
    params(
        ("account_id" = String, Path, description = "ID da conta Stellar")
    ),
    responses(
        (status = 200, description = "Conta retornada com sucesso", body = Account)
    )
)]
#[get("/accounts/{account_id}")]
async fn get_account(account_id: web::Path<String>) -> impl Responder {
    let client = Client::new();
    let url = format!("{}/accounts/{}", get_base_url().await, account_id.into_inner());

    match client.get(&url).send().await {
        Ok(response) => match response.json::<Account>().await {
            Ok(account) => HttpResponse::Ok().json(account),
            Err(_) => HttpResponse::InternalServerError().body("Erro ao deserializar a resposta."),
        },
        Err(_) => HttpResponse::InternalServerError().body("Erro ao acessar a API da Stellar."),
    }
}

/// Obtém as transações de uma conta
#[utoipa::path(
    get,
    path = "/accounts/{account_id}/transactions",
    params(
        ("account_id" = String, Path, description = "ID da conta Stellar")
    ),
    responses(
        (status = 200, description = "Transações retornadas com sucesso", body = Vec<Transaction>)
    )
)]
#[get("/accounts/{account_id}/transactions")]
async fn get_account_transactions(account_id: web::Path<String>) -> impl Responder {
    let client = Client::new();
    let url = format!("{}/accounts/{}/transactions", get_base_url().await, account_id.into_inner());

    match client.get(&url).send().await {
        Ok(response) => match response.json::<Vec<Transaction>>().await {
            Ok(transactions) => HttpResponse::Ok().json(transactions),
            Err(_) => HttpResponse::InternalServerError().body("Erro ao deserializar transações."),
        },
        Err(_) => HttpResponse::InternalServerError().body("Erro ao acessar a API da Stellar."),
    }
}

/// Obtém detalhes de uma oferta
#[utoipa::path(
    get,
    path = "/offers/{offer_id}",
    params(
        ("offer_id" = String, Path, description = "ID da oferta")
    ),
    responses(
        (status = 200, description = "Oferta retornada com sucesso", body = Offer)
    )
)]
#[get("/offers/{offer_id}")]
async fn get_offer(offer_id: web::Path<String>) -> impl Responder {
    let client = Client::new();
    let url = format!("{}/offers/{}", get_base_url().await, offer_id.into_inner());

    match client.get(&url).send().await {
        Ok(response) => match response.json::<Offer>().await {
            Ok(offer) => HttpResponse::Ok().json(offer),
            Err(_) => HttpResponse::InternalServerError().body("Erro ao deserializar a oferta."),
        },
        Err(_) => HttpResponse::InternalServerError().body("Erro ao acessar a API da Stellar."),
    }
}

/// Obtém informações de um ledger
#[utoipa::path(
    get,
    path = "/ledgers/{sequence}",
    params(
        ("sequence" = u32, Path, description = "Número do ledger")
    ),
    responses(
        (status = 200, description = "Ledger retornado com sucesso", body = Ledger)
    )
)]
#[get("/ledgers/{sequence}")]
async fn get_ledger(sequence: web::Path<u32>) -> impl Responder {
    let client = Client::new();
    let url = format!("{}/ledgers/{}", get_base_url().await, sequence.into_inner());

    match client.get(&url).send().await {
        Ok(response) => match response.json::<Ledger>().await {
            Ok(ledger) => HttpResponse::Ok().json(ledger),
            Err(_) => HttpResponse::InternalServerError().body("Erro ao deserializar o ledger."),
        },
        Err(_) => HttpResponse::InternalServerError().body("Erro ao acessar a API da Stellar."),
    }
}
