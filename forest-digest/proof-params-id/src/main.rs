use storage_proofs::compound_proof::{self, CompoundProof};
use storage_proofs::hasher::{ Hasher};
use storage_proofs::porep::stacked::{ self, StackedCompound, StackedDrg };

use filecoin-proofs::constants::{ DefaultPieceHasher };
use filecoin-proofs::parameters::setup_params;
use filecoin-proofs::types::{ PaddedBytesAmount, PoRepConfig },


fn main() {
    let compound_setup_params = compound_proof::SetupParams {
        vanilla_params: setup_params(
            PaddedBytesAmount::from(porep_config),
            usize::from(PoRepProofPartitions::from(porep_config)),
            porep_config.porep_id,
        )?,
        partitions: Some(usize::from(PoRepProofPartitions::from(porep_config))),
        priority: false,
    };

    let compound_public_params = <StackedCompound<Tree, DefaultPieceHasher> as CompoundProof<
        StackedDrg<Tree, DefaultPieceHasher>,
        _,
    >>::setup(&compound_setup_params)?;
}
