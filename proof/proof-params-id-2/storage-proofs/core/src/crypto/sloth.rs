use ff::Field;
use paired::bls12_381::Fr;

/// Sloth based encoding.
#[inline]
pub fn encode(key: &Fr, plaintext: &Fr) -> Fr {
    let mut ciphertext = *plaintext;

    ciphertext.add_assign(key); // c + k
    ciphertext
}

/// Sloth based decoding.
#[inline]
pub fn decode(key: &Fr, ciphertext: &Fr) -> Fr {
    let mut plaintext = *ciphertext;

    plaintext.sub_assign(key); // c - k

    plaintext
}
