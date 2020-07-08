use bellperson::{Circuit};
use log::info;
use paired::bls12_381::Bls12;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};


pub trait ParameterSetMetadata {
    fn identifier(&self) -> String;
    fn sector_size(&self) -> u64;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheEntryMetadata {
    pub sector_size: u64,
}

pub trait CacheableParameters<C, P>
where
    C: Circuit<Bls12>,
    P: ParameterSetMetadata,
{
    fn cache_prefix() -> String;

    fn cache_meta(pub_params: &P) -> CacheEntryMetadata {
        CacheEntryMetadata {
            sector_size: pub_params.sector_size(),
        }
    }

    fn cache_identifier(pub_params: &P) -> String {
        let param_identifier = pub_params.identifier();
        info!("parameter set identifier for cache: {}", param_identifier);
        let mut hasher = Sha256::default();
        hasher.input(&param_identifier.into_bytes());
        let circuit_hash = hasher.result();
        format!(
            "{}-{:02x}",
            Self::cache_prefix(),
            circuit_hash
        )
    }
}
