use crate::block::{Block};
use num_bigint::{BigInt};
pub struct ProofOfWork{
    block:Block,
    target:BigInt,
}
