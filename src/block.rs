use std::vec;

use crate::transaction::{self, TXInput, TXOutput, Transaction};
use crate::proof_of_work::{ProofOfWork};
use serde::{Deserialize, Serialize};
use sled::IVec;

#[derive(Clone,Deserialize,Serialize)]
pub struct Block {
    timestamp: i64,         //it will strore the integer value when block was created
    pre_block_hash: String, //hash value of the previous hash
    hash: String,           //hash value of the cuurent blocka
    transactions: Vec<Transaction>, //vector that hold  various transaction
    nonce: i64, //value that miner changes the value while mining a block to finad a hash
    height: usize, //the position of current block
}
impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
        let pow = ProofOfWork::new_proof_of_work(block.clone);
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        return block;
    }
    pub fn deserialize(bytes:&[u8])->Block{
        bincode::deserialize(bytes).unwrap()
    }

    pub fn serialize(&self)-> Vec<u8>{
        bincode::serialize(self).unwrap().to_vec()

    }

    pub fn get_transaction(&self) -> &[Transaction] {
        self.transactions.as_slice()
        
    }

    pub  fn get_pre_block_hash(&self)->String{
        self.pre_block_hash.clone()
    }
    pub fn get_hash_bytes(&self)->Vec<u8>{
        self.hash.as_bytes().to_vec()
    }
    pub fn get_hash(&self)->&str{
        self.hash.as_str()
    }
    pub fn get_timestamp(&self)->i64{
        self.timestamp
    }
    pub fn get_height(&self)->usize{
        self.height
    }

    pub fn hash_transaction(&self)-> Vec<u8>{
        let mut txhashs=vec![];
        for transaction in  &self.transactions{
            txhashs.extend(transaction.get_id());
        }
        crate::sha256_digest(txhashs.as_slice())
    }

    pub  fn generate_genesis_block(transaction:&Transaction)->Block{
        let transaction=vec![transaction.clone()];
        return Block::new_block(String::from("None"), &transaction, 0);

    }


}
impl From<Block> for  IVec {
    fn from(b: Block) -> Self {
        let bytes=bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
    
}
