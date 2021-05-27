use crate::grpc::tran::{TranRequest, TranResponse};
use crate::block_chain::chain::Chain;
use crate::block_chain::func;
use std::sync::MutexGuard;
use tonic::{Status, Response};
use crate::block_chain::tx::Tx;
use crate::block_chain::utxo::Utxo;
use crate::block_chain::utxos::Utxos;

pub fn main(req:TranRequest,chain:&mut MutexGuard<Chain>) ->Result<Response<TranResponse>, Status>{
    let pk = req.clone().private_key;
    let amount = req.clone().amount.parse::<f32>().unwrap();
    let from = req.clone().from;
    let to = req.clone().to;
    let token_name = req.clone().token;
    let from_balance = chain.get_balance(&from.clone(),&token_name.clone());
    if {(from.clone().len(),to.clone().len())==(32,32)}==false {
        Ok(Response::new(TranResponse{
            hash:"00000000000000000000000000000000".to_string()
        }))
    }else if from_balance < amount || amount <= 0.0 {
        Ok(Response::new(TranResponse{
            hash:"00000000000000000000000000000001".to_string()
        }))
    }else if {func::address(&pk.clone())==from}==false {
        Ok(Response::new(TranResponse{
            hash:"00000000000000000000000000000002".to_string()
        }))
    }else{
        let tx_from = Tx::new(from,token_name.clone(),amount);
        let tx_to = Tx::new(to,token_name.clone(),amount);
        let utxo = Utxo::new(tx_from,tx_to);
        let utxos = Utxos::new(vec![utxo]);
        let hash = chain.add_block(utxos);
        let response:TranResponse =TranResponse{
            hash:hash.to_string()
        };
        Ok(Response::new(response))
    }
}
