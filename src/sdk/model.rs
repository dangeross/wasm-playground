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