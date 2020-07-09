use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub struct PaddedBytesAmount(pub u64);

impl From<PaddedBytesAmount> for u64 {
    fn from(n: PaddedBytesAmount) -> Self {
        n.0
    }
}

impl From<PaddedBytesAmount> for usize {
    fn from(n: PaddedBytesAmount) -> Self {
        n.0 as usize
    }
}