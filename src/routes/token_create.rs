use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use spl_token::instruction as token_instruction;
use crate::models::responses::{SuccessResponse, ErrorResponse, InstructionResponse, AccountMetaResponse};
use crate::utils::solana::decode_pubkey;
use base58::ToBase58;

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    mint_authority: String,
    mint: String,
    decimals: u8,
}

#[post("/token/create")]
pub async fn create_token(req: web::Json<CreateTokenRequest>) -> HttpResponse {
    let mint_authority = match decode_pubkey(&req.mint_authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid mint authority pubkey".to_string(),
            });
        }
    };

    let mint = match decode_pubkey(&req.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid mint pubkey".to_string(),
            });
        }
    };

    let token_program_id = spl_token::id();
    
    
    let instruction = token_instruction::initialize_mint(
        &token_program_id,
        &mint,
        &mint_authority,
        None,
        req.decimals,
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