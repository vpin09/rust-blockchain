pub struct Block{
    Timestamp: i64,
    pre_block_hash :String,
    hash :String,
    transaction : Vec<Transaction>,
    nonce : i64,
    height : usize,
}