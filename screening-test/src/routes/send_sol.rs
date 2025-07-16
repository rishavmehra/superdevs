use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::system_instruction;
use crate::models::responses::{SuccessResponse, ErrorResponse};
use crate::utils::solana::decode_pubkey;
use base58::ToBase58;

#[derive(Deserialize)]
pub struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolInstructionResponse {
    pub program_id: String,
    pub accounts: Vec<String>, 
    pub instruction_data: String,
}

#[post("/send/sol")]
pub async fn send_sol(req: web::Json<SendSolRequest>) -> HttpResponse {
    if req.lamports == 0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid sender public key".to_string(),
        });
    }

    let from_pubkey = match decode_pubkey(&req.from) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid sender public key".to_string(),
            });
        }
    };

    let to_pubkey = match decode_pubkey(&req.to) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid sender public key".to_string(),
            });
        }
    };

    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, req.lamports);
    
    let accounts: Vec<String> = instruction.accounts
        .iter()
        .map(|meta| meta.pubkey.to_string())
        .collect();

    let response = SuccessResponse {
        success: true,
        data: SendSolInstructionResponse {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data: instruction.data.to_base58(),
        },
    };
    
    HttpResponse::Ok().json(response)
}