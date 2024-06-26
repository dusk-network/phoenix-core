// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! This module implements the ElGamal asymmetric cipher. It allows to
//! encrypt and decrypt.
//!
//! Reference: https://link.springer.com/chapter/10.1007/3-540-39568-7_2

use dusk_jubjub::{JubJubExtended, JubJubScalar, GENERATOR};

/// Encrypts a JubJubExtended plaintext given a public key and a fresh random
/// number 'r'.
///
/// ## Return
/// Returns a ciphertext (JubJubExtended, JubJubExtended).
pub fn encrypt(
    public_key: &JubJubExtended,
    plaintext: &JubJubExtended,
    r: &JubJubScalar,
) -> (JubJubExtended, JubJubExtended) {
    let ciphertext_1 = GENERATOR * r;
    let ciphertext_2 = plaintext + public_key * r;

    (ciphertext_1, ciphertext_2)
}

/// Decrypts a ciphertext given a secret key.
///
/// ## Return
/// Returns a JubJubExtended plaintext.
pub fn decrypt(
    secret_key: &JubJubScalar,
    ciphertext: &(JubJubExtended, JubJubExtended),
) -> JubJubExtended {
    let ciphertext_1 = ciphertext.0;
    let ciphertext_2 = ciphertext.1;

    // return the plaintext
    ciphertext_2 - ciphertext_1 * secret_key
}
