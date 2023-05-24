// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Substrate specific configuration

use super::{
    extrinsic_params::{BaseExtrinsicParams, BaseExtrinsicParamsBuilder},
    Config,
};
use codec::Encode;

pub use crate::utils::{AccountId32, MultiAddress, MultiSignature};
use crate::SubstrateConfig;
pub use primitive_types::{H256, U256};

/// Default set of commonly used types by Substrate runtimes.
// Note: We only use this at the type level, so it should be impossible to
// create an instance of it.
pub enum CordConfig {}

impl Config for CordConfig {
    type Index = <SubstrateConfig as Config>::Index;
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = MultiAddress<Self::AccountId, ()>;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = CordExtrinsicParams<Self>;
}

// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a cord node.
pub type CordExtrinsicParams<T> = BaseExtrinsicParams<T, PlainTip>;

/// A builder which leads to [`CordExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type CordExtrinsicParamsBuilder<T> = BaseExtrinsicParamsBuilder<T, PlainTip>;

// Because Era is one of the args to our extrinsic params.
pub use super::extrinsic_params::Era;


/// A tip payment.
#[derive(Copy, Clone, Debug, Default, Encode)]
pub struct PlainTip {
    #[codec(compact)]
    tip: u128,
}

impl PlainTip {
    /// Create a new tip of the amount provided.
    pub fn new(amount: u128) -> Self {
        PlainTip { tip: amount }
    }
}

impl From<u128> for PlainTip {
    fn from(n: u128) -> Self {
        PlainTip::new(n)
    }
}



