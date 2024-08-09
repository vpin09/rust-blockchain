use core::hash;
use std::borrow::Borrow;
use data_encoding::HEXLOWER;

use crate::block::{Block};
use num_bigint::{BigInt};
pub struct ProofOfWork{
    block:Block,
    target:BigInt,
}
impl ProofOfWork {
    pub fn run(&self)->(i64,String){
        let mut nonce=0;
        let mut  hash=Vec::new();
        println!("mining the block");
        while nonce <MAX_NONCE {
            let data=self.prepare_data(nonce);
            hash=crate::sha256_digest(data.as_slice());
            let hash_int=BigInt::from_bytes_be(num_bigint::Sign::Plus, hash.as_slice());
            if hash_int.lt(self.target.borrow()){
                println!("{}",HEXLOWER.encode(hash.as_slice()));
                break;
            }else {
                nonce +=1;
            }
        }
        println!();
        return (nonce,HEXLOWER.encode(hash.as_slice()));

        
    }
    
}
