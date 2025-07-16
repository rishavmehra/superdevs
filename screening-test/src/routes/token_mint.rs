use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use spl_token::instruction as token_instruction;
use crate::models::responses::{SuccessResponse, ErrorResponse, InstructionResponse, AccountMetaResponse};
use crate::utils::solana::decode_pubkey;
use base58::ToBase58;

#[derive(Deserialize)]
pub struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

#[post("/token/mint")]
pub async fn mint_token(req: web::Json<MintTokenRequest>) -> HttpResponse {
    let mint = match decode_pubkey(&req.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid mint pubkey".to_string(),
            });
        }
    };

    let destination = match decode_pubkey(&req.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid destination pubkey".to_string(),
            });
        }
    };

    let authority = match decode_pubkey(&req.authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid authority pubkey".to_string(),
            });
        }
    };

    let token_program_id = spl_token::id();
    
    let instruction = token_instruction::mint_to(
        &token_program_id,
        &mint,
        &destination,
        &authority,
        &[],  
        req.amount,
    ).unwrap();

    let accounts: Vec<AccountMetaResponse> = instruction.accounts
        .iter()
        .map(|meta| AccountMetaResponse {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect();

    let response = SuccessResponse {
        success: true,
        data: InstructionResponse {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data: instruction.data.to_base58(),
        },
    };
    
    HttpResponse::Ok().json(response)
}
