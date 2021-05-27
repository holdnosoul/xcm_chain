// use util::greeter::{GetBlockRequest, GetBlockResponse};
// use util::chain::Chain;
// use std::sync::MutexGuard;
// use tonic::{Status, Response};
//
// pub fn main(req:GetBlockRequest,chain:&mut MutexGuard<Chain>) ->Result<Response<GetBlockResponse>, Status>{
//     let param_type:String = req.clone().address.to_string();
//     let param:String = req.clone().token.to_string();
//     let balance = chain.get_block(&address,&token);
//     let rep = GetBlockResponse { balance:balance.to_string() };
//     Ok(Response::new(rep))
// }
