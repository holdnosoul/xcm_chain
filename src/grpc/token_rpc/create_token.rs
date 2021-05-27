use crate::grpc::token::{TokenRequest, TokenResponse};
use std::sync::MutexGuard;
use tonic::{Status, Response};
use crate::block_chain::chain::Chain;
use crate::block_chain::tx::Tx;
use crate::block_chain::utxo::Utxo;
use crate::block_chain::utxos::Utxos;

pub fn main(req:TokenRequest,chain:&mut MutexGuard<Chain>) ->Result<Response<TokenResponse>, Status>{
    let amount = req.clone().amount.parse::<f32>().unwrap();
    let from = "00000000000000000000000000000000".to_string();
    let to = req.clone().address;
    let token_name = req.clone().token;
    println!("Received request : {:?}", req.clone());
    if req.private_key == "123456" && amount as f64 > 0.0  {
        let tx_from = Tx::new(from,token_name.clone(),amount);
        let tx_to = Tx::new(to,token_name.clone(),amount);
        let utxo = Utxo::new(tx_from,tx_to);
        let utxos = Utxos{utxos:vec![utxo]};
        let hash = chain.add_block(utxos);
        let response:TokenResponse =TokenResponse{
            hash:hash.to_string()
        };
        Ok(Response::new(response))
    }else{
        Ok(Response::new(TokenResponse{
            hash:"error".to_string()
        }))
    }
}
