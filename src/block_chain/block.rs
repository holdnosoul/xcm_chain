use serde::{Serialize, Deserialize};
use std::io::Write;
use std::fs::File;
use std::io::BufReader;
use crate::block_chain::utxos::Utxos;
use crate::block_chain::func;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Block {
    pub height: u128,
    pub hash: String,
    pub time: u64,
    pub transaction: Utxos,
    pub prehash: String,
}




impl Block {
    pub fn new(utxos: Utxos,last_prehash:&str,last_height:u128) -> Block{
        let height = last_height + 1;
        let prehash = last_prehash.clone().to_string();
        let time: u64 = func::timestamp();
        let transaction = utxos;
        let str = format!("{:?}{:?}{:?}{:?}", time.to_string(), height.to_string(), transaction.to_string(), prehash);
        let hash = func::sha256(&str);

        Block{
            height,
            hash,
            time,
            transaction,
            prehash,
        }
    }

    #[warn(dead_code)]
    pub fn get_hash(&self)->String{
        let str = format!("{:?}{:?}{:?}{:?}", self.time.to_string(), self.height.to_string(), self.transaction.to_string(), self.prehash);
        func::sha256(&str)
    }


    #[warn(dead_code)]
    pub fn check_block(&self)->bool{
        self.hash==Self::get_hash(&self)
    }

    pub fn to_string(&self) -> String {
        let res = serde_json::to_string(&self).unwrap();
        res
    }

    pub fn read_block(path: &str) -> Block {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let obj: Block = serde_json::from_reader(reader).unwrap();
        return obj;
    }

    pub fn write(&self){
        let path = format!("./data/{}.block",self.height);
        let mut file = std::fs::File::create(path).expect("create failed");
        let str = self.to_string();
        file.write_all(str.as_bytes()).expect("write failed");
    }
}
