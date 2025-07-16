use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};
use thiserror::Error;
use std::str::FromStr;
use base58::{FromBase58, ToBase58};

#[derive(Error, Debug)]
pub enum SolanaError {
    #[error("Invalid base58 encoding")]
    InvalidBase58,
    
    #[error("Invalid public key: {0}")]
    InvalidPubkey(String),
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
}

pub fn decode_keypair(secret: &str) -> Result<Keypair, SolanaError> {
    let bytes = secret.from_base58().map_err(|_| SolanaError::InvalidBase58)?;
    Keypair::from_bytes(&bytes).map_err(|_| SolanaError::InvalidBase58)
}

pub fn encode_keypair(keypair: &Keypair) -> String {
    keypair.to_bytes().to_base58()
}

pub fn decode_pubkey(pubkey: &str) -> Result<Pubkey, SolanaError> {
    Pubkey::from_str(pubkey).map_err(|e| SolanaError::InvalidPubkey(e.to_string()))
}

pub fn sign_message(message: &[u8], keypair: &Keypair) -> String {
    let signature = keypair.sign_message(message);
    signature.as_ref().to_base58()
}

pub fn verify_signature(
    message: &[u8], 
    signature_str: &str, 
    pubkey: &Pubkey
) -> Result<bool, SolanaError> {
    let signature_bytes = signature_str.from_base58()
        .map_err(|_| SolanaError::InvalidSignature)?;
    
    if signature_bytes.len() != 64 {
        return Err(SolanaError::InvalidSignature);
    }
    
    let mut signature_array = [0u8; 64];
    signature_array.copy_from_slice(&signature_bytes);
    
    let signature = Signature::from(signature_array);
    Ok(signature.verify(pubkey.as_ref(), message))
}