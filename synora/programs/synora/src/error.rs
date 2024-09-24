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
}