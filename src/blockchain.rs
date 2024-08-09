use std::sync::{Arc, RwLock};
use std::env::current_dir;

use  sled::{Db,Tree};

const BLOCKS_TREE:&str="blocks";
const TIP_BLOCK_HASH_KEY:&str="tip_block_hash";

use crate::block::{self, Block};
use crate::transaction::{Transaction};
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
}