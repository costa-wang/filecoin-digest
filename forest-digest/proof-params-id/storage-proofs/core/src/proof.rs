use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::error::Result;

/// The ProofScheme trait provides the methods that any proof scheme needs to implement.
pub trait ProofScheme<'a> {
    type PublicParams: Clone;
    type SetupParams: Clone;
    type PublicInputs: Clone;
    type PrivateInputs;
    type Proof: Clone + Serialize + DeserializeOwned;
    type Requirements: Default;

    /// setup is used to generate public parameters from setup parameters in order to specialize
    /// a ProofScheme to the specific parameters required by a consumer.
    fn setup(_: &Self::SetupParams) -> Result<Self::PublicParams>;
}