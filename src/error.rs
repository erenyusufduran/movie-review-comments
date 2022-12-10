use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Account not intiialized yet")]
    UninitializedAccount,
    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,
    #[error("Input data exceeds max length")]
    InvalidDataLength,
    #[error("Rating greater than 5 or less than 1")]
    InvalidRating,
    #[error("Accounts are not same.")]
    IncorrectAccountError,
}

impl From<ReviewError> for ProgramError {
    fn from(err: ReviewError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
