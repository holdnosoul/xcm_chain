use crate::grpc::greeter::{GetBalanceRequest, GetBalanceResponse};
use std::sync::MutexGuard;
use tonic::{Status, Response};
use crate::block_chain::chain::Chain;

pub fn main(req:GetBalanceRequest,chain:&mut MutexGuard<Chain>) ->Result<Response<GetBalanceResponse>, Status>{
    let address:String = req.clone().address.to_string();
    let token:String = req.clone().token.to_string();
    let balance = chain.get_balance(&address,&token);
    let rep = GetBalanceResponse { balance:balance.to_string() };
    Ok(Response::new(rep))
}
