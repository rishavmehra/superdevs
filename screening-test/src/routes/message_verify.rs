use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use crate::models::responses::{SuccessResponse, ErrorResponse, VerificationResponse};
use crate::utils::solana::{decode_pubkey, verify_signature};

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    message: String,
    signature: String,
    pubkey: String,
}

#[post("/message/verify")]
pub async fn verify_message(req: web::Json<VerifyMessageRequest>) -> HttpResponse {
    if req.message.is_empty() || req.signature.is_empty() || req.pubkey.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Missing required fields".to_string(),
        });
    }

    let pubkey = match decode_pubkey(&req.pubkey) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid public key".to_string(),
            });
        }
    };

    let message_bytes = req.message.as_bytes();
    
    match verify_signature(message_bytes, &req.signature, &pubkey) {
        Ok(valid) => {
            let response = SuccessResponse {
                success: true,
                data: VerificationResponse {
                    valid,
                    message: req.message.clone(),
                    pubkey: req.pubkey.clone(),
                },
            };
            HttpResponse::Ok().json(response)
        },
        Err(_) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid signature format".to_string(),
            })
        }
    }
}
