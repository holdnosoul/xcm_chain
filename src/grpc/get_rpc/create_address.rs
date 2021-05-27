use crate::grpc::greeter::CreateAddressResponse;
use tonic::{Status, Response};
use crate::block_chain::func;

pub fn main() ->Result<Response<CreateAddressResponse>, Status>{
    let pk = func::private_key();
    let rep = CreateAddressResponse {
        private_key:pk.clone(),
        address:func::address(&pk)
    };
    Ok(Response::new(rep))
}
