use std::sync::{Arc, RwLock};
use std::env::current_dir;

use  sled::{Db,Tree};

const BLOCKS_TREE:&str="blocks";
const TIP_BLOCK_HASH_KEY:&str="tip_block_hash";

use crate::block::{self, Block};
use crate::transaction::{self, Transaction};
pub struct Blockchain{
    tip_hash:Arc<RwLock<String>>,
    db: Db,
}
impl Blockchain{
    pub fn create_blockchain(genesis_address:&str)->Blockchain{
        let db=sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree=db.open_tree(BLOCKS_TREE).unwrap();
        let data=blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_hash;
        if data.is_none(){
            let coinbase_tx=Transaction::new_coinbase_tx(genesis_address);
            let block=Block::generate_genesis_block(&coinbase_tx);
            Self::update_blocks_tree(&blocks_tree,&block);
            tip_hash=String::from(block.get_hash());

        }else {
            tip_hash=String::from_utf8(data.unwrap().to_vec()).unwrap();
        }
        Blockchain{
            tip_hash:Arc::new(RwLock::new(tip_hash)),
            db 
        }

    }    
    pub fn new_blockchain()->Blockchain{
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree=db.open_tree(BLOCKS_TREE).unwrap();
        let tip_bytes=blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap().expect("No existing blockchain found create one");\
        let tip_hash=String::from_utf8(tip_bytes.to_vec()).unwrap();
        Blockchain{
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db
        }
    }
    pub fn get_db(&self)->&Db{
        &self.db
    }
    pub fn get_tip_hash(&self)->String {
        self.tip_hash.read().unwrap().clone()
    }
    pub fn set_tip_hash(&self,new_tip_hash:&str){
        let mut tip_hash=self.tip_hash.write().unwrap();
        *tip_hash=String::from(new_tip_hash)
    }
    pub fn mine_block(&mut self,transactions:Vec<Transaction>)->Block {
        for transaction in transactions{
            if transaction.verify(self) == false {
                panic!("ERROR: Invalid Transaction")
            }

            
        }
        let best_height=self.get_best_height();
        let block=Block::new_block(self.get_tip_hash(), &transactions, best_height+1);
        let blocks_hash=block.get_hash();
        let blocks_tree=self.db.open_tree(BLOCKS_TREE).unwrap();
        Self::update_blocks_tree(&blocks_tree,&block);
        self.set_tip_hash(blocks_hash);
        block
        
    }

}
pub struct BlockchainIterator{
    db:Db,
    current_hash:String,
}
impl BlockchainIterator {
    fn new(tip_hash:String,db:Db)-> BlockchainIterator {

    }
    pub fn iterator(&self)->BlockchainIterator {
        BlockchainIterator::new(self.get_tip_hash(),self.db.clone())
        
    }
    pub fn next(&mut self)->Option<Block>{

    }
    
}