//! Implements support for the pallet_identity module.

use super::balances::Balances;
use codec::Encode;

pub use pallet_identity::{
    Registration,

};

/// The subset of the `frame::Trait` that a client must implement.
#[module]
pub trait Identity: Balances {}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Store)]
/// Information that is pertinent to identify the entity behind an account.
pub struct IdentityOfStore<T: Identity> {
    #[store(returns = Option<Registration<T::Balance>>)]
    /// Information that is pertinent to identify the entity behind an account.
    pub account: T::AccountId,
}
