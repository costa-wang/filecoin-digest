use std::cmp::{max, min};
use std::marker::PhantomData;

use anyhow::ensure;
use generic_array::typenum;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use sha2::{Digest, Sha256};

use crate::crypto::{derive_porep_domain_seed, DRSAMPLE_DST};
use crate::error::*;
use crate::fr32::bytes_into_fr_repr_safe;
use crate::hasher::{Hasher, PoseidonArity};
use crate::parameter_cache::ParameterSetMetadata;
use crate::util::{data_at_node_offset, NODE_SIZE};

pub const PARALLEL_MERKLE: bool = true;

/// The base degree used for all DRG graphs. One degree from this value is used to ensure that a
/// given node always has its immediate predecessor as a parent, thus ensuring unique topological
/// ordering of the graph nodes.
pub const BASE_DEGREE: usize = 6;

/// A depth robust graph.
pub trait Graph<H: Hasher>: ::std::fmt::Debug + Clone + PartialEq + Eq {
    type Key: std::fmt::Debug;

    /// Returns the expected size of all nodes in the graph.
    fn expected_size(&self) -> usize {
        self.size() * NODE_SIZE
    }

    /// Returns the merkle tree depth.
    fn merkle_tree_depth<U: 'static + PoseidonArity>(&self) -> u64 {
        graph_height::<U>(self.size()) as u64
    }

    /// Returns a sorted list of all parents of this node. The parents may be repeated.
    ///
    /// If a node doesn't have any parents, then this vector needs to return a vector where
    /// the first element is the requested node. This will be used as indicator for nodes
    /// without parents.
    ///
    /// The `parents` parameter is used to store the result. This is done fore performance
    /// reasons, so that the vector can be allocated outside this call.
    fn parents(&self, node: usize, parents: &mut [u32]) -> Result<()>;

    /// Returns the size of the graph (number of nodes).
    fn size(&self) -> usize;

    /// Returns the number of parents of each node in the graph.
    fn degree(&self) -> usize;

    fn new(
        nodes: usize,
        base_degree: usize,
        expansion_degree: usize,
        porep_id: [u8; 32],
    ) -> Result<Self>;
    fn seed(&self) -> [u8; 28];

    /// Creates the encoding key.
    /// The algorithm for that is `Sha256(id | encodedParentNode1 | encodedParentNode1 | ...)`.
    fn create_key(
        &self,
        id: &H::Domain,
        node: usize,
        parents: &[u32],
        parents_data: &[u8],
        exp_parents_data: Option<&[u8]>,
    ) -> Result<Self::Key>;
}

pub fn graph_height<U: typenum::Unsigned>(number_of_leafs: usize) -> usize {
    merkletree::merkle::get_merkle_tree_row_count(number_of_leafs, U::to_usize())
}

/// Bucket sampling algorithm.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct BucketGraph<H: Hasher> {
    nodes: usize,
    base_degree: usize,
    seed: [u8; 28],
    _h: PhantomData<H>,
}

impl<H: Hasher> ParameterSetMetadata for BucketGraph<H> {
    fn identifier(&self) -> String {
        // NOTE: Seed is not included because it does not influence parameter generation.
        format!(
            "drgraph::BucketGraph{{size: {}; degree: {}; hasher: {}}}",
            self.nodes,
            self.degree(),
            H::name(),
        )
    }

    fn sector_size(&self) -> u64 {
        (self.nodes * NODE_SIZE) as u64
    }
}

impl<H: Hasher> Graph<H> for BucketGraph<H> {
    type Key = H::Domain;

    fn create_key(
        &self,
        id: &H::Domain,
        node: usize,
        parents: &[u32],
        base_parents_data: &[u8],
        _exp_parents_data: Option<&[u8]>,
    ) -> Result<Self::Key> {
        let mut hasher = Sha256::new();
        hasher.input(AsRef::<[u8]>::as_ref(id));

        // The hash is about the parents, hence skip if a node doesn't have any parents
        if node != parents[0] as usize {
            for parent in parents.iter() {
                let offset = data_at_node_offset(*parent as usize);
                hasher.input(&base_parents_data[offset..offset + NODE_SIZE]);
            }
        }

        let hash = hasher.result();
        Ok(bytes_into_fr_repr_safe(hash.as_ref()).into())
    }

    #[inline]
    fn parents(&self, node: usize, parents: &mut [u32]) -> Result<()> {
        let m = self.degree();

        match node {
            // There are special cases for the first and second node: the first node self
            // references, the second node only references the first node.
            0 | 1 => {
                // Use the degree of the current graph (`m`) as `parents.len()` might be bigger than
                // that (that's the case for Stacked Graph).
                for parent in parents.iter_mut().take(m) {
                    *parent = 0;
                }
                Ok(())
            }
            _ => {
                // DRG node indexes are guaranteed to fit within a `u32`.
                let node = node as u32;

                let mut seed = [0u8; 32];
                seed[..28].copy_from_slice(&self.seed);
                seed[28..].copy_from_slice(&node.to_le_bytes());
                let mut rng = ChaCha8Rng::from_seed(seed);

                let m_prime = m - 1;
                // Large sector sizes require that metagraph node indexes are `u64`.
                let metagraph_node = node as u64 * m_prime as u64;
                let n_buckets = (metagraph_node as f64).log2().ceil() as u64;

                for parent in parents.iter_mut().take(m_prime) {
                    let bucket_index = (rng.gen::<u64>() % n_buckets) + 1;
                    let largest_distance_in_bucket = min(metagraph_node, 1 << bucket_index);
                    let smallest_distance_in_bucket = max(2, largest_distance_in_bucket >> 1);

                    // Add 1 becuase the number of distances in the bucket is inclusive.
                    let n_distances_in_bucket =
                        largest_distance_in_bucket - smallest_distance_in_bucket + 1;

                    let distance =
                        smallest_distance_in_bucket + (rng.gen::<u64>() % n_distances_in_bucket);

                    let metagraph_parent = metagraph_node - distance;

                    // Any metagraph node mapped onto the DRG can be safely cast back to `u32`.
                    let mapped_parent = (metagraph_parent / m_prime as u64) as u32;

                    *parent = if mapped_parent == node {
                        node - 1
                    } else {
                        mapped_parent
                    };
                }

                parents[m_prime] = node - 1;
                Ok(())
            }
        }
    }

    #[inline]
    fn size(&self) -> usize {
        self.nodes
    }

    /// Returns the degree of the graph.
    #[inline]
    fn degree(&self) -> usize {
        self.base_degree
    }

    fn seed(&self) -> [u8; 28] {
        self.seed
    }

    fn new(
        nodes: usize,
        base_degree: usize,
        expansion_degree: usize,
        porep_id: [u8; 32],
    ) -> Result<Self> {
        ensure!(expansion_degree == 0, "Expension degree must be zero.");

        // The number of metagraph nodes must be less than `2u64^54` as to not incur rounding errors
        // when casting metagraph node indexes from `u64` to `f64` during parent generation.
        let m_prime = base_degree - 1;
        let n_metagraph_nodes = nodes as u64 * m_prime as u64;
        ensure!(
            n_metagraph_nodes <= 1u64 << 54,
            "The number of metagraph nodes must be precisely castable to `f64`"
        );

        let drg_seed = derive_drg_seed(porep_id);

        Ok(BucketGraph {
            nodes,
            base_degree,
            seed: drg_seed,
            _h: PhantomData,
        })
    }
}

pub fn derive_drg_seed(porep_id: [u8; 32]) -> [u8; 28] {
    let mut drg_seed = [0; 28];
    let raw_seed = derive_porep_domain_seed(DRSAMPLE_DST, porep_id);
    drg_seed.copy_from_slice(&raw_seed[..28]);
    drg_seed
}
