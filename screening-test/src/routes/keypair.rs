use actix_web::{post, HttpResponse};
use solana_sdk::{
    signer::{keypair::Keypair, Signer},
};
use crate::models::responses::{SuccessResponse, KeypairResponse};
use crate::utils::solana::encode_keypair;

#[post("/keypair")]
pub async fn generate_keypair() -> HttpResponse {
    let keypair = Keypair::new();
    let response = SuccessResponse {
        success: true,
        data: KeypairResponse {
            pubkey: keypair.pubkey().to_string(),
            secret: encode_keypair(&keypair),
        },
    };
    
    HttpResponse::Ok().json(response)
}