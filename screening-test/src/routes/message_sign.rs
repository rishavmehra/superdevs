use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use solana_sdk::signer::Signer;
use crate::models::responses::{SuccessResponse, ErrorResponse, SignatureResponse};
use crate::utils::solana::{decode_keypair, sign_message as solana_sign_message};

#[derive(Deserialize)]
pub struct SignMessageRequest {
    message: String,
    secret: String,
}

#[post("/message/sign")]
pub async fn sign_message_route(req: web::Json<SignMessageRequest>) -> HttpResponse {
    if req.message.is_empty() || req.secret.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Missing required fields".to_string(),
        });
    }

    let keypair = match decode_keypair(&req.secret) {
        Ok(keypair) => keypair,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid secret key".to_string(),
            });
        }
    };

    let message_bytes = req.message.as_bytes();
    let signature = solana_sign_message(message_bytes, &keypair);
    
    let response = SuccessResponse {
        success: true,
        data: SignatureResponse {
            signature,
            pubkey: keypair.pubkey().to_string(),
            message: req.message.clone(),
        },
    };
    
    HttpResponse::Ok().json(response)
}
