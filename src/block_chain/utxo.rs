use serde::{Serialize, Deserialize};
use crate::block_chain::tx::Tx;


#[derive(Debug,Serialize, Deserialize,PartialEq, Clone)]
pub struct Utxo{
    pub input:Tx,
    pub output:Tx
}

impl Utxo {
    pub fn new(input: Tx, output:Tx) -> Utxo {
        Utxo{
            input,output
        }
    }

    pub fn to_string(&self){
        serde_json::to_string(&self).unwrap();
    }
}
