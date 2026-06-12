use soroban_sdk::{contracterror, symbol_short, Symbol};

/// Error codes for the Medical Record Hash Registry contract.
///
/// Error Ranges:
/// - 100-199: Access Control
/// - 200-299: Input Validation
/// - 300-399: Lifecycle & State
/// - 400-499: Entity Existence
/// - 500-599: Financial & Resource
/// - 700-799: Cross-Chain
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // --- Access Control (100-199) ---
    /// Caller is not authorized to perform this action
    Unauthorized = 100,

    // --- Input Validation (200-299) ---
    /// Invalid identifier provided
    InvalidId = 206,
    /// Invalid cryptographic signature
    InvalidSignature = 207,
    /// Invalid record hash format
    InvalidRecordHash = 251,

    // --- Lifecycle & State (300-399) ---
    /// Contract has not been initialized
    NotInitialized = 300,
    /// Contract has already been initialized
    AlreadyInitialized = 301,
    /// Contract is paused; store_record is suspended
    ContractPaused = 302,
    /// Operation deadline exceeded
    DeadlineExceeded = 306,

    // --- Entity Existence (400-499) ---
    /// Duplicate record hash rejected for this patient
    DuplicateRecord = 402,
    /// Record not found for the given hash or patient
    RecordNotFound = 403,

    // --- Financial & Resource (500-599) ---
    /// Insufficient funds for operation
    InsufficientFunds = 500,
    /// Storage is full
    StorageFull = 502,

    // --- Cross-Chain (700-799) ---
    /// Cross-chain operation timed out
    CrossChainTimeout = 702,
}

pub fn get_suggestion(error: Error) -> Symbol {
    match error {
        Error::Unauthorized => symbol_short!("CHK_AUTH"),
        Error::NotInitialized => symbol_short!("INIT_CTR"),
        Error::AlreadyInitialized => symbol_short!("ALREADY"),
        Error::ContractPaused | Error::DeadlineExceeded => symbol_short!("RE_TRY_L"),
        Error::InvalidId | Error::DuplicateRecord | Error::RecordNotFound => {
            symbol_short!("CHK_ID")
        },
        Error::InsufficientFunds => symbol_short!("ADD_FUND"),
        Error::StorageFull => symbol_short!("CLN_OLD"),
        Error::CrossChainTimeout => symbol_short!("RE_TRY_L"),
        _ => symbol_short!("CONTACT"),
    }
}
