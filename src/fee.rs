// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! Fee module contains the logic related to `Fee` and `Remainder` structure

use dusk_pki::{Ownable, PublicSpendKey, StealthAddress};
use poseidon252::sponge::sponge::sponge_hash;
use rand_core::{CryptoRng, RngCore};

use crate::{BlsScalar, JubJubScalar};

mod remainder;
pub use remainder::Remainder;

/// The Fee structure
#[derive(Clone, Copy, Debug)]
pub struct Fee {
    /// The gas limit set for the fee
    pub gas_limit: u64,
    /// the gas price set for the fee
    pub gas_price: u64,
    pub(crate) stealth_address: StealthAddress,
}

impl PartialEq for Fee {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

impl Eq for Fee {}

impl Fee {
    /// Create a new Fee with inner randomness
    pub fn new<R: RngCore + CryptoRng>(
        rng: &mut R,
        gas_limit: u64,
        gas_price: u64,
        psk: &PublicSpendKey,
    ) -> Self {
        let r = JubJubScalar::random(rng);

        Self::deterministic(gas_limit, gas_price, &r, psk)
    }

    /// Create a new Fee without inner randomness
    pub fn deterministic(
        gas_limit: u64,
        gas_price: u64,
        r: &JubJubScalar,
        psk: &PublicSpendKey,
    ) -> Self {
        let stealth_address = psk.gen_stealth_address(r);

        Fee {
            gas_limit,
            gas_price,
            stealth_address,
        }
    }

    /// Return a hash represented by `H(gas_limit, gas_price, H([pskr]))`
    pub fn hash(&self) -> BlsScalar {
        let pk_r = self.stealth_address().pk_r().to_hash_inputs();

        sponge_hash(&[
            BlsScalar::from(self.gas_limit),
            BlsScalar::from(self.gas_price),
            pk_r[0],
            pk_r[1],
        ])
    }

    /// Generates a remainder from the fee and the given gas consumed
    pub fn gen_remainder(&self, gas_consumed: u64) -> Remainder {
        // Consuming more gas than the limit provided should never
        // occur, and it's not responsability of the `Remainder` to
        // check that.
        // Here defensively ensure it's not panicking, capping the gas
        // consumed to the gas limit.
        let gas_consumed = std::cmp::min(gas_consumed, self.gas_limit);
        let gas_changes = (self.gas_limit - gas_consumed) * self.gas_price;

        Remainder {
            gas_changes,
            stealth_address: self.stealth_address,
        }
    }
}

impl Ownable for Fee {
    fn stealth_address(&self) -> &StealthAddress {
        &self.stealth_address
    }
}
