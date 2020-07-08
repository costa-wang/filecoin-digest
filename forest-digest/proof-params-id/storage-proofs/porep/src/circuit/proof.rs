use std::marker::PhantomData;

use bellperson::{Circuit};
use paired::bls12_381::{Bls12};
use storage_proofs_core::{
    drgraph::Graph,
    hasher::{Hasher},
    merkle::{MerkleTreeTrait},
    parameter_cache::{CacheableParameters, ParameterSetMetadata},
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