use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use serde::Serialize;
use spl_token::instruction as token_instruction;
use spl_associated_token_account::get_associated_token_address;
use crate::models::responses::{SuccessResponse, ErrorResponse};
use crate::utils::solana::decode_pubkey;
use base58::ToBase58;

#[derive(Deserialize)]
pub struct SendTokenRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}


#[derive(Serialize)]
pub struct SendTokenAccountResponse {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Serialize)]
pub struct SendTokenInstructionResponse {
    pub program_id: String,
    pub accounts: Vec<SendTokenAccountResponse>,
    pub instruction_data: String,
}

#[post("/send/token")]
pub async fn send_token(req: web::Json<SendTokenRequest>) -> HttpResponse {
    // Validate inputs
    if req.amount == 0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Amount must be greater than 0".to_string(),
        });
    }

    let destination = match decode_pubkey(&req.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid destination pubkey".to_string(),
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

    let owner = match decode_pubkey(&req.owner) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid owner pubkey".to_string(),
            });
        }
    };

    // Derive the token account addresses
    let source_token_account = get_associated_token_address(
        &owner,
        &mint,
    );
    
    let destination_token_account = get_associated_token_address(
        &destination,
        &mint,
    );

    let token_program_id = spl_token::id();
    
    let instruction = token_instruction::transfer(
        &token_program_id,
        &source_token_account,
        &destination_token_account,
        &owner,
        &[],  
        req.amount,
    ).unwrap();

    let accounts: Vec<SendTokenAccountResponse> = instruction.accounts
        .iter()
        .map(|meta| SendTokenAccountResponse {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
        })
        .collect();

    let response = SuccessResponse {
        success: true,
        data: SendTokenInstructionResponse {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data: instruction.data.to_base58(),
        },
    };
    
    HttpResponse::Ok().json(response)
}
