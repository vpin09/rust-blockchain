use std::sync::{Arc, RwLock};
use  sled::{Db,Tree};

use crate::transaction::{Transaction};
pub struct Blockchain{
    tip_hash:Arc<RwLock<String>>,
    db: Db,
}
impl Blockchain{
    pub fn create_blockchain(genesis:&str)->Blockchain{
        let db=sled::open(current_dir().unwrap().join).unwrap();
        let blocks_tree=db.open_tree(BLOCKS_TREE).unwrap();
        let data=blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_hash;
        if data.is_none(){
            let coinbase_tx=Transaction::new
        }

    }    
}