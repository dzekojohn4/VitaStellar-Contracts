use soroban_sdk::{contracterror, symbol_short, Symbol};

/// Error codes for the Patient Consent Management contract.
/// 
/// Error Ranges:
/// - 100-199: Access Control & Authorization
/// - 200-299: Input Validation
/// - 300-399: Lifecycle & State
/// - 400-499: Entity Existence & Business Rules
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // --- Access Control (100-199) ---
    /// Caller is not authorized to perform this action
    Unauthorized = 100,

    // --- Input Validation (200-299) ---
    /// Patient address is invalid or missing
    InvalidPatient = 210,
    /// Provider address is invalid or missing
    InvalidProvider = 211,

    // --- Lifecycle & State (300-399) ---
    /// Contract has not been initialized
    NotInitialized = 300,
    /// Contract has already been initialized
    AlreadyInitialized = 301,
    /// Contract is paused; operations are suspended
    ContractPaused = 302,

    // --- Entity Existence & Business Rules (400-499) ---
    /// Consent record not found for the given patient/provider pair
    ConsentNotFound = 406,
    /// Consent already exists and is active for this pair
    ConsentAlreadyExists = 460,
    /// Expiry timestamp is in the past or at current time
    InvalidExpiry = 470,
    /// Batch size exceeds maximum allowed (50 providers per batch)
    BatchTooLarge = 471,
    /// Invalid input provided (e.g., empty batch list)
    InvalidInput = 472,
}

#[allow(dead_code)]
pub fn get_suggestion(error: Error) -> Symbol {
    match error {
        Error::Unauthorized => symbol_short!("CHK_AUTH"),
        Error::NotInitialized => symbol_short!("INIT_CTR"),
        Error::AlreadyInitialized => symbol_short!("ALREADY"),
        Error::InvalidPatient | Error::InvalidProvider => symbol_short!("CHK_ID"),
        Error::ConsentNotFound => symbol_short!("CHK_ID"),
        Error::ConsentAlreadyExists => symbol_short!("ALREADY"),
        Error::BatchTooLarge | Error::InvalidInput => symbol_short!("CHK_ID"),
    }
}
