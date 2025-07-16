use serde::Serialize;
use solana_sdk::instruction::AccountMeta;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct AccountMetaResponse {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct InstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaResponse>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SignatureResponse {
    pub signature: String,
    pub pubkey: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct VerificationResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

impl From<&AccountMeta> for AccountMetaResponse {
    fn from(account_meta: &AccountMeta) -> Self {
        Self {
            pubkey: account_meta.pubkey.to_string(),
            is_signer: account_meta.is_signer,
            is_writable: account_meta.is_writable,
        }
    }
}