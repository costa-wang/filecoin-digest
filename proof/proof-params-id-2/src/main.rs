use sha2::{Digest, Sha256};
use itertools::Itertools;

use filecoin_proofs_v1::storage_proofs::compound_proof::{self, CompoundProof};
use filecoin_proofs_v1::storage_proofs::proof::{ProofScheme};
use filecoin_proofs_v1::storage_proofs::hasher::{ Hasher};
use filecoin_proofs_v1::storage_proofs::porep::stacked::{ self, StackedCircuit, StackedCompound, StackedDrg };
use filecoin_proofs_v1::storage_proofs::porep::drg::{ DrgPoRepCompound };
use filecoin_proofs_v1::storage_proofs::merkle::{ MerkleTreeWrapper };
use filecoin_proofs_v1::storage_proofs::parameter_cache::{CacheableParameters};

use filecoin_proofs_v1::{ MerkleTreeTrait };
use filecoin_proofs_v1::with_shape;
use filecoin_proofs_v1::constants::{ DefaultPieceHasher };
use filecoin_proofs_v1::parameters::{ public_params, setup_params };
use filecoin_proofs_v1::types::{ PoRepConfig, PoRepProofPartitions, PaddedBytesAmount};
use filecoin_proofs_api::registry::{ RegisteredSealProof };

fn print_type_name<T>(_: &T) {
    println!("{}", std::any::type_name::<T>() );
}

// fn get_param_id<Tree: 'static + MerkleTreeTrait>(porep_config:PoRepConfig) -> String {
//     let compound_setup_params = compound_proof::SetupParams{
//         vanilla_params: setup_params(
//             PaddedBytesAmount::from(porep_config),
//             usize::from(PoRepProofPartitions::from(porep_config)),
//             porep_config.porep_id,
//         ).unwrap(),
//         partitions: Some(usize::from(PoRepProofPartitions::from(porep_config))),
//         priority: false,
//     };

//     //print_type_name(&compound_setup_params);
//     //print_type_name(&compound_setup_params.vanilla_params);

//     let compound_public_params = <StackedCompound<Tree, DefaultPieceHasher> as CompoundProof<
//         StackedDrg<Tree, DefaultPieceHasher>,
//         _,
//     >>::setup(&compound_setup_params).unwrap();

//     print_type_name(&compound_public_params);
//     print_type_name(&compound_public_params.vanilla_params);
// }

fn get_param_id<Tree: 'static + MerkleTreeTrait>(porep_config:PoRepConfig) -> String {
    let params = public_params::<Tree>(
        porep_config.sector_size.into(),
        porep_config.partitions.into(),
        porep_config.porep_id,
    ).unwrap();
    let id = <StackedCompound<Tree, DefaultPieceHasher> as CacheableParameters<
            StackedCircuit<Tree, DefaultPieceHasher>,
            _,
        >>::cache_identifier(&params);
    id
}


fn main() {
    let register_proof = RegisteredSealProof::StackedDrg2KiBV1;
    let porep_config = register_proof.as_v1_config();
    let id = with_shape!(register_proof.sector_size().0,get_param_id,porep_config);
    println!("{}",id);
}
