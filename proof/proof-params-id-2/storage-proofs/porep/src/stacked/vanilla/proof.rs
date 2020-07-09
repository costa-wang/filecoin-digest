use std::marker::PhantomData;

use storage_proofs_core::{
    hasher::{ Hasher },
    merkle::{MerkleTreeTrait},
};

#[derive(Debug)]
pub struct StackedDrg<'a, Tree: 'a + MerkleTreeTrait, G: 'a + Hasher> {
    _a: PhantomData<&'a Tree>,
    _b: PhantomData<&'a G>,
}