use std::marker::PhantomData;

use anyhow::ensure;
use bellperson::gadgets::num;
use bellperson::{Circuit, ConstraintSystem, SynthesisError};
use paired::bls12_381::{Bls12, Fr};
use storage_proofs_core::{
    compound_proof::{CircuitComponent, CompoundProof},
    drgraph::Graph,
    error::Result,
    fr32::u64_into_fr,
    gadgets::constraint,
    gadgets::por::PoRCompound,
    hasher::{HashFunction, Hasher},
    merkle::{BinaryMerkleTree, MerkleTreeTrait},
    parameter_cache::{CacheableParameters, ParameterSetMetadata},
    por,
    proof::ProofScheme,
    util::reverse_bit_numbering,
};

use super::params::Proof;
use crate::stacked::StackedDrg;

#[allow(dead_code)]
pub struct StackedCompound<Tree: MerkleTreeTrait, G: Hasher> {
    partitions: Option<usize>,
    _t: PhantomData<Tree>,
    _g: PhantomData<G>,
}

impl<C: Circuit<Bls12>, P: ParameterSetMetadata, Tree: MerkleTreeTrait, G: Hasher>
    CacheableParameters<C, P> for StackedCompound<Tree, G>
{
    fn cache_prefix() -> String {
        format!(
            "stacked-proof-of-replication-{}-{}",
            Tree::display(),
            G::name()
        )
    }
}