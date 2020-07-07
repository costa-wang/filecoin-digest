use serde::{Deserialize, Serialize};
use storage_proofs_core::{
    parameter_cache::ParameterSetMetadata,
};

use super::{ graph::StackedBucketGraph, LayerChallenges };
#[derive(Debug, Clone)]
pub struct SetupParams {
    // Number of nodes
    pub nodes: usize,

    // Base degree of DRG
    pub degree: usize,

    pub expansion_degree: usize,

    pub porep_id: [u8; 32],
    pub layer_challenges: LayerChallenges,
}

#[derive(Debug)]
pub struct PublicParams<Tree>
where
    Tree: 'static + MerkleTreeTrait,
{
    pub graph: StackedBucketGraph<Tree::Hasher>,
    pub layer_challenges: LayerChallenges,
    _t: PhantomData<Tree>,
}

impl<Tree> Clone for PublicParams<Tree>
where
    Tree: MerkleTreeTrait,
{
    fn clone(&self) -> Self {
        Self {
            graph: self.graph.clone(),
            layer_challenges: self.layer_challenges.clone(),
            _t: Default::default(),
        }
    }
}

impl<Tree> PublicParams<Tree>
where
    Tree: MerkleTreeTrait,
{
    pub fn new(graph: StackedBucketGraph<Tree::Hasher>, layer_challenges: LayerChallenges) -> Self {
        PublicParams {
            graph,
            layer_challenges,
            _t: PhantomData,
        }
    }
}

impl<Tree> ParameterSetMetadata for PublicParams<Tree>
where
    Tree: MerkleTreeTrait,
{
    fn identifier(&self) -> String {
        format!(
            "layered_drgporep::PublicParams{{ graph: {}, challenges: {:?}, tree: {} }}",
            self.graph.identifier(),
            self.layer_challenges,
            Tree::display()
        )
    }

    fn sector_size(&self) -> u64 {
        self.graph.sector_size()
    }
}

impl<'a, Tree> From<&'a PublicParams<Tree>> for PublicParams<Tree>
where
    Tree: MerkleTreeTrait,
{
    fn from(other: &PublicParams<Tree>) -> PublicParams<Tree> {
        PublicParams::new(other.graph.clone(), other.layer_challenges.clone())
    }
}