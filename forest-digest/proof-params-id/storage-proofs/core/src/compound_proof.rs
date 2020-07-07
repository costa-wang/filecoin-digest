use paired::bls12_381::{Bls12};

use crate::error::Result;
use crate::parameter_cache::{CacheableParameters, ParameterSetMetadata};

use crate::proof::ProofScheme;


#[derive(Clone)]
pub struct SetupParams<'a, S: ProofScheme<'a>> {
    pub vanilla_params: <S as ProofScheme<'a>>::SetupParams,
    pub partitions: Option<usize>,
    /// High priority (always runs on GPU) == true
    pub priority: bool,
}

#[derive(Clone)]
pub struct PublicParams<'a, S: ProofScheme<'a>> {
    pub vanilla_params: S::PublicParams,
    pub partitions: Option<usize>,
    pub priority: bool,
}

pub trait CircuitComponent {
    type ComponentPrivateInputs: Default + Clone;
}

/// The CompoundProof trait bundles a proof::ProofScheme and a bellperson::Circuit together.
/// It provides methods equivalent to those provided by proof::ProofScheme (setup, prove, verify).
/// See documentation at proof::ProofScheme for details.
/// Implementations should generally only need to supply circuit and generate_public_inputs.
/// The remaining trait methods are used internally and implement the necessary plumbing.
pub trait CompoundProof<'a, S: ProofScheme<'a>, C: Circuit<Bls12> + CircuitComponent + Send>
where
    S::Proof: Sync + Send,
    S::PublicParams: ParameterSetMetadata + Sync + Send,
    S::PublicInputs: Clone + Sync,
    Self: CacheableParameters<C, S::PublicParams>,
{
    // setup is equivalent to ProofScheme::setup.
    fn setup(sp: &SetupParams<'a, S>) -> Result<PublicParams<'a, S>> {
        Ok(PublicParams {
            vanilla_params: S::setup(&sp.vanilla_params)?,
            partitions: sp.partitions,
            priority: sp.priority,
        })
    }
}