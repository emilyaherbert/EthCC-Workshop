library errors;

pub enum InitializationError {
    CannotReinitialize: (),
    ContractNotInitialized: (),
}

pub enum UserError {
    AmountCannotBeZero: (),
    IncorrectAssetSent: (),
    InvalidNumber: (),
    InsufficientBalance: (),
    //InvalidId: (),
    //VoteAmountCannotBeZero: (),
}
