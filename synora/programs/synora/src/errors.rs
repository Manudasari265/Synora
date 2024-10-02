use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Deposit amount equal to the odds")]
    AmountNotSufficient,
    #[msg("Invalid Odds")]
    InvalidOdds,
    #[msg("You can't join because event alreday started")]
    EventAlreadyStarted,
    #[msg("Can't cancel event started")]
    EventCantCancel,
    #[msg("You can't perform this action")]
    UnauthorizedAccess,
    #[msg("The bet has not ended yet.")]
    BetNotEndedYet,
    #[msg("The bet has not been resolved yet. Please wait until the bet is completed.")]
    BetNotResolvedYet,
    #[msg("Invalid fee percentage")]
    InvalidFeePercentage,
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
    #[msg("Failed to transfer funds")]
    TransferFailed,
    #[msg("Invalid bet status")]
    InvalidBetStatus,
    #[msg("Price feed error")]
    PriceFeedError,
    #[msg("Invalid start time")]
    InvalidStartTime,
    #[msg("Invalid end time")]
    InvalidEndTime,
    #[msg("Invalid deadline")]
    InvalidDeadline,
    #[msg("Bet is already resolved")]
    BetAlreadyResolved,
    #[msg("Bet not accepted")]
    BetNotAccepted,
    #[msg("Resolver Feed does not match")]
    MismatchFeed,
    #[msg("Switchbaord: NoValueFound")]
    NoValueFound,
    #[msg("Switchbaord: NoFeedData")]
    NoFeedData,
    #[msg("Switchbaord: PriceConversionOverflow")]
    PriceConversionOverflow,
}