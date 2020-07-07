#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use anyhow::ensure;
use storage_proofs_core::{
    crypto::{
        derive_porep_domain_seed,
        feistel::{self, FeistelPrecomputed},
        FEISTEL_DST,
    },
    drgraph::BASE_DEGREE,
    drgraph::{BucketGraph, Graph},
    error::Result,
    hasher::Hasher,
    parameter_cache::ParameterSetMetadata,
};

/// The expansion degree used for Stacked Graphs.
pub const EXP_DEGREE: usize = 8;

pub(crate) const DEGREE: usize = BASE_DEGREE + EXP_DEGREE;

#[derive(Clone)]
pub struct StackedGraph<H, G>
where
    H: Hasher,
    G: Graph<H> + 'static,
{
    expansion_degree: usize,
    base_graph: G,
    pub(crate) feistel_keys: [feistel::Index; 4],
    feistel_precomputed: FeistelPrecomputed,
    id: String,
    _h: PhantomData<H>,
}

impl<H, G> std::fmt::Debug for StackedGraph<H, G>
where
    H: Hasher,
    G: Graph<H> + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StackedGraph")
            .field("expansion_degree", &self.expansion_degree)
            .field("base_graph", &self.base_graph)
            .field("feistel_precomputed", &self.feistel_precomputed)
            .field("id", &self.id)
            .finish()
    }
}

pub type StackedBucketGraph<H> = StackedGraph<H, BucketGraph<H>>;

pub fn derive_feistel_keys(porep_id: [u8; 32]) -> [u64; 4] {
    let mut feistel_keys = [0u64; 4];
    let raw_seed = derive_porep_domain_seed(FEISTEL_DST, porep_id);
    feistel_keys[0] = u64::from_le_bytes(raw_seed[0..8].try_into().unwrap());
    feistel_keys[1] = u64::from_le_bytes(raw_seed[8..16].try_into().unwrap());
    feistel_keys[2] = u64::from_le_bytes(raw_seed[16..24].try_into().unwrap());
    feistel_keys[3] = u64::from_le_bytes(raw_seed[24..32].try_into().unwrap());
    feistel_keys
}

impl<H, G> StackedGraph<H, G>
where
    H: Hasher,
    G: Graph<H> + ParameterSetMetadata + Sync + Send,
{
    pub fn new(
        base_graph: Option<G>,
        nodes: usize,
        base_degree: usize,
        expansion_degree: usize,
        porep_id: [u8; 32],
    ) -> Result<Self> {
        assert_eq!(base_degree, BASE_DEGREE);
        assert_eq!(expansion_degree, EXP_DEGREE);
        ensure!(nodes <= std::u32::MAX as usize, "too many nodes");

        let base_graph = match base_graph {
            Some(graph) => graph,
            None => G::new(nodes, base_degree, 0, porep_id)?,
        };
        let bg_id = base_graph.identifier();

        let feistel_keys = derive_feistel_keys(porep_id);

        let res = StackedGraph {
            base_graph,
            id: format!(
                "stacked_graph::StackedGraph{{expansion_degree: {} base_graph: {} }}",
                expansion_degree, bg_id,
            ),
            expansion_degree,
            feistel_keys,
            feistel_precomputed: feistel::precompute((expansion_degree * nodes) as feistel::Index),
            _h: PhantomData,
        };

        Ok(res)
    }
}