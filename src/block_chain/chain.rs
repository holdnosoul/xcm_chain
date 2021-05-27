#[warn(dead_code)]

use serde::{Serialize, Deserialize};
use std::fs;
use crate::block_chain::block::Block;
use crate::block_chain::utxo::Utxo;
use crate::block_chain::func;
use crate::block_chain::tx::Tx;
use crate::block_chain::utxos::Utxos;


#[derive(Debug, Serialize, Deserialize, Clone,PartialEq)]
pub struct Chain {
    pub blocks: Vec<Block>
}

impl Chain {
    fn genesis() -> Block{
        let utxo:Utxo = Utxo {input: Tx { address: "00000000000000000000000000000000".to_string(), token: "0000000000000000000000000000000000000000000000000000000000000000".to_string(), amount: 0.0, time: 0 }, output: Tx { address: "00000000000000000000000000000000".to_string(), token: "0000000000000000000000000000000000000000000000000000000000000000".into(), amount: 0.0, time: 0 } };
        let height = 1;
        let prehash = "00000000000000000000000000000000".to_string();
        let time: u64 = 0;
        let transaction = Utxos::new(vec![utxo]);
        let str = format!("{:?}{:?}{:?}{:?}", time.to_string(), height.to_string(), transaction.to_string(), prehash);
        let hash = func::sha256(&str);
        let block = Block{
            height,
            hash,
            time,
            transaction,
            prehash,
        };
        block.write();
        block
    }



    pub fn new() -> Self {
        let paths = fs::read_dir("./data").unwrap();
        let mut count:u128 = 0;
        let mut blocks = Vec::new();
        for _path in paths {
            count = count+1;
            let block = Block::read_block(&format!("./data/{}.block",&count));
            println!("load...{:?}.{:?}",count,block);
            blocks.push(block);
        }
        if blocks.len()==0 {
            blocks.push(Self::genesis());
        }
        Chain { blocks }
    }
    pub fn add_block(&mut self,utxos:Utxos)->String{
        let last_block = Self::last_block( self);
        let block = Block::new(utxos,&last_block.hash,last_block.height);
        let hash_str = block.clone().hash;
        self.blocks.push(block.clone());
        block.write();
        return hash_str;
    }



    pub fn last_block(&mut self)->&Block{
        &self.blocks[self.blocks.len()-1]
    }

    pub fn to_string(&mut self) -> String {
        let res = serde_json::to_string(&self).unwrap();
        res
    }

    pub fn check_chain(&self)->bool{
        if self.blocks.len()>1 {
            for (index, block) in self.blocks.iter().enumerate() {
                println!("blocks at index {}: {:?}", index, block);
                if index > 0 {
                    if self.blocks[index].prehash == self.blocks[index - 1].hash && block.check_block() {
                        continue;
                    } else {
                        return false
                    }
                }else { continue; }
            }
            return true
        }else{
            Block::check_block(&self.blocks[0])
        }
    }

    pub fn get_balance(&mut self,_address:&str,_token:&str)->f32{
        let blocks =  self.blocks.clone();
        let mut in_total = 0.0;
        let mut out_total = 0.0;
        for block in blocks.iter() {
            for utxo in block.transaction.utxos.iter(){
                if _address == utxo.input.address && _token == utxo.input.token {
                    in_total = in_total + utxo.input.amount;
                }
                if _address == utxo.output.address && _token == utxo.output.token {
                    out_total = out_total + utxo.output.amount;
                }
            }

        }
        println!("in total:{:?}",in_total);
        println!("out total:{:?}",out_total);
        out_total - in_total
    }

    #[allow(dead_code)]
    pub fn get_address_tx(&mut self,_address:&str,_token:&str)->Vec<Tx>{
        let blocks =  self.blocks.clone();
        let mut txs:Vec<Tx> = vec![];
        for block in blocks.iter() {
            for utxo in block.transaction.utxos.iter() {
                if _address == utxo.input.address {
                    txs.push(utxo.clone().input);
                }
                if _address == utxo.output.address {
                    txs.push(utxo.clone().output);
                }
            }
        }
        txs
    }



    //
    // pub fn get_address_tx(&mut self,address:&str,token:&str){
    //
    // }
    //
    // pub fn get_token_total(&mut self,address:&str,token:&str){
    //
    // }
}
