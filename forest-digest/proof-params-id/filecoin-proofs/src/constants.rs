use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;
use storage_proofs::hasher::Hasher;

pub const SECTOR_SIZE_2_KIB: u64 = 1 << 11;
pub const SECTOR_SIZE_4_KIB: u64 = 1 << 12;
pub const SECTOR_SIZE_16_KIB: u64 = 1 << 14;
pub const SECTOR_SIZE_32_KIB: u64 = 1 << 15;
pub const SECTOR_SIZE_8_MIB: u64 = 1 << 23;
pub const SECTOR_SIZE_16_MIB: u64 = 1 << 24;
pub const SECTOR_SIZE_512_MIB: u64 = 1 << 29;
pub const SECTOR_SIZE_1_GIB: u64 = 1 << 30;
pub const SECTOR_SIZE_32_GIB: u64 = 1 << 35;
pub const SECTOR_SIZE_64_GIB: u64 = 1 << 36;


pub const DRG_DEGREE: usize = storage_proofs::drgraph::BASE_DEGREE;
pub const EXP_DEGREE: usize = storage_proofs::porep::stacked::EXP_DEGREE;

lazy_static! {
    pub static ref POREP_MINIMUM_CHALLENGES: RwLock<HashMap<u64, u64>> = RwLock::new(
        [
            (SECTOR_SIZE_2_KIB, 2),
            (SECTOR_SIZE_4_KIB, 2),
            (SECTOR_SIZE_16_KIB, 2),
            (SECTOR_SIZE_32_KIB, 2),
            (SECTOR_SIZE_8_MIB, 2),
            (SECTOR_SIZE_16_MIB, 2),
            (SECTOR_SIZE_512_MIB, 2),
            (SECTOR_SIZE_1_GIB, 2),
            (SECTOR_SIZE_32_GIB, 176),
            (SECTOR_SIZE_64_GIB, 176),
        ]
        .iter()
        .copied()
        .collect()
    );
    pub static ref POREP_PARTITIONS: RwLock<HashMap<u64, u8>> = RwLock::new(
        [
            (SECTOR_SIZE_2_KIB, 1),
            (SECTOR_SIZE_4_KIB, 1),
            (SECTOR_SIZE_16_KIB, 1),
            (SECTOR_SIZE_32_KIB, 1),
            (SECTOR_SIZE_8_MIB, 1),
            (SECTOR_SIZE_16_MIB, 1),
            (SECTOR_SIZE_512_MIB, 1),
            (SECTOR_SIZE_1_GIB, 1),
            (SECTOR_SIZE_32_GIB, 10),
            (SECTOR_SIZE_64_GIB, 10),
        ]
        .iter()
        .copied()
        .collect()
    );
    pub static ref LAYERS: RwLock<HashMap<u64, usize>> = RwLock::new(
        [
            (SECTOR_SIZE_2_KIB, 2),
            (SECTOR_SIZE_4_KIB, 2),
            (SECTOR_SIZE_16_KIB, 2),
            (SECTOR_SIZE_32_KIB, 2),
            (SECTOR_SIZE_8_MIB, 2),
            (SECTOR_SIZE_16_MIB, 2),
            (SECTOR_SIZE_512_MIB, 2),
            (SECTOR_SIZE_1_GIB, 2),
            (SECTOR_SIZE_32_GIB, 11),
            (SECTOR_SIZE_64_GIB, 11),
        ]
        .iter()
        .copied()
        .collect()
    );
    // These numbers must match those used for Window PoSt scheduling in the miner actor.
    // Please coordinate changes with actor code.
    // https://github.com/filecoin-project/specs-actors/blob/master/actors/abi/sector.go
    
}


/// The hasher used for creating comm_d.
pub type DefaultPieceHasher = storage_proofs::hasher::Sha256Hasher;


/// Calls a function with the type hint of the sector shape matching the provided sector.
/// Panics if provided with an unknown sector size.
#[macro_export]
macro_rules! with_shape {
    ($size:expr, $f:ident) => {
        with_shape!($size, $f,)
    };
    ($size:expr, $f:ident, $($args:expr,)*) => {
        match $size {
            _x if $size == $crate::constants::SECTOR_SIZE_2_KIB => {
              $f::<$crate::constants::SectorShape2KiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_4_KIB => {
              $f::<$crate::constants::SectorShape4KiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_16_KIB => {
              $f::<$crate::constants::SectorShape16KiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_32_KIB => {
              $f::<$crate::constants::SectorShape32KiB>($($args),*)
            },
            _xx if $size == $crate::constants::SECTOR_SIZE_8_MIB => {
              $f::<$crate::constants::SectorShape8MiB>($($args),*)
            },
            _xx if $size == $crate::constants::SECTOR_SIZE_16_MIB => {
              $f::<$crate::constants::SectorShape16MiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_512_MIB => {
              $f::<$crate::constants::SectorShape512MiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_1_GIB => {
              $f::<$crate::constants::SectorShape1GiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_32_GIB => {
              $f::<$crate::constants::SectorShape32GiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_64_GIB => {
              $f::<$crate::constants::SectorShape64GiB>($($args),*)
            },
            _ => panic!("unsupported sector size: {}", $size),
        }
    };
    ($size:expr, $f:ident, $($args:expr),*) => {
        with_shape!($size, $f, $($args,)*)
    };
}
