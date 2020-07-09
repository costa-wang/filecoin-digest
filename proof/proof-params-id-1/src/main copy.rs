use storage_proofs::compound_proof::{self, CompoundProof};
use storage_proofs::hasher::{ Hasher};
use storage_proofs::porep::stacked::{ self, StackedCompound, StackedDrg };

use filecoin_proofs::constants::{ DefaultPieceHasher };
use filecoin_proofs::parameters::setup_params;
use filecoin_proofs::types::{ PoRepConfig, PoRepProofPartitions, PaddedBytesAmount};
use filecoin_proofs_api::registry::{ RegisteredSealProof };


fn main() {
    let two_kb = RegisteredSealProof::StackedDrg2KiBV1;
    let porep_config = two_kb.as_v1_config();
    let compound_setup_params = compound_proof::SetupParams {
        vanilla_params: setup_params(
            PaddedBytesAmount(porep_config.sector_size.0),
            usize::from(porep_config.partitions.0),
            porep_config.porep_id,
        ).unwrap(),
        partitions: Some(porep_config.partitions.0 as usize),
        priority: false,
    };

    // let compound_public_params = <StackedCompound<Tree, DefaultPieceHasher> as CompoundProof<
    //     StackedDrg<Tree, DefaultPieceHasher>,
    //     _,
    // >>::setup(&compound_setup_params)?;
}
