use maybe_sync::{MaybeSend, MaybeSync};

#[derive(Clone, Debug, PartialEq)]
pub struct Payment {
    pub id: String,
}

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub line: String,
    pub level: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SdkEvent {
    PaymentFailed { details: Payment },
    PaymentPending { details: Payment },
    PaymentRefundable { details: Payment },
    PaymentRefunded { details: Payment },
    PaymentRefundPending { details: Payment },
    PaymentSucceeded { details: Payment },
    PaymentWaitingConfirmation { details: Payment },
    PaymentWaitingFeeAcceptance { details: Payment },
    Synced,
}

pub trait EventListener: MaybeSend + MaybeSync {
    fn on_event(&self, e: SdkEvent);
}

#[derive(thiserror::Error, Debug)]
pub enum SignerError {
    #[error("Signer error: {err}")]
    Generic { err: String },
}

#[allow(dead_code)]
pub trait Signer: MaybeSend + MaybeSync {
    fn xpub(&self) -> Result<Vec<u8>, SignerError>;

    fn derive_xpub(&self, derivation_path: String) -> Result<Vec<u8>, SignerError>;

    fn sign_ecdsa(&self, msg: Vec<u8>, derivation_path: String) -> Result<Vec<u8>, SignerError>;

    fn sign_ecdsa_recoverable(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError>;

    fn slip77_master_blinding_key(&self) -> Result<Vec<u8>, SignerError>;

    fn hmac_sha256(&self, msg: Vec<u8>, derivation_path: String) -> Result<Vec<u8>, SignerError>;

    fn ecies_encrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError>;

    fn ecies_decrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, SignerError>;
}