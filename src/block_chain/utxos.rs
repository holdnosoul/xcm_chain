use crate::block_chain::utxo::Utxo;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq)]
pub struct Utxos{
    pub utxos: Vec<Utxo>
}

impl Utxos {
    pub fn new(utxos:Vec<Utxo>) -> Utxos {
        Utxos{
            utxos
        }
    }

    pub fn to_string(&self){
        serde_json::to_string(&self).unwrap();
    }
}

