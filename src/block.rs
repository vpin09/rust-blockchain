
use crate::transaction::{Transaction,TXInput,TXOutput};
pub struct Block {
    Timestamp: i64,         //it will strore the integer value when block was created
    pre_block_hash: String, //hash value of the previous hash
    hash: String,           //hash value of the cuurent blocka
    transaction: Vec<Transaction>, //vector that hold  various transaction
    nonce: i64, //value that miner changes the value while mining a block to finad a hash
    height: usize, //the position of current block
}
impl Block {
    pub fn new_block(pre_block_hash: String, transaction: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transaction: transaction.to_vec(),
            nonce: 0,
            height,
        };
        let pow = ProofOfWork::new_proof_of_work(block.clone);
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        return block;
    }
}
