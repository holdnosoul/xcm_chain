use tonic::{Request, Response, Status};

use crate::grpc::greeter;
use greeter::greeter_server::{Greeter};
use greeter::{GetBalanceRequest,GetBalanceResponse};
use greeter::{CreateAddressRequest,CreateAddressResponse};

use crate::grpc::tran;
use tran::tran_server::{Tran};
use tran::{TranRequest,TranResponse};

use crate::grpc::token;
use token::token_server::{Token};
use token::{TokenRequest,TokenResponse};

use crate::block_chain::BLOCK_CHAIN;

#[derive(Debug, Default,Copy, Clone)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {

    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let ref mut chain = BLOCK_CHAIN.lock().unwrap();
        let req = request.into_inner().clone();
        println!("Received request : {:?}", req);
        crate::grpc::get_rpc::get_balance::main(req,chain)
    }

    // async fn get_block(
    //     &self,
    //     request: Request<GetBlockRequest>,
    // ) -> Result<Response<GetBlockResponse>, Status> {
    //     let ref mut chain = (*BLOCK_CHAIN).lock().unwrap();
    //     let req = request.into_inner().clone();
    //     println!("Received request : {:?}", req);
    //     crate::lib::get_rpc::get_block::main(req,chain)
    // }

    async fn create_address(
        &self,
        _request: Request<CreateAddressRequest>,
    ) -> Result<Response<CreateAddressResponse>, Status> {
        crate::grpc::get_rpc::create_address::main()
    }
}

#[tonic::async_trait]
impl Tran for MyGreeter {
    async fn tran_rpc(
        &self,
        request: Request<TranRequest>,
    ) -> Result<Response<TranResponse>, Status> {
        let ref mut chain = (*BLOCK_CHAIN).lock().unwrap();
        let req = request.into_inner().clone();
        println!("Received request : {:?}", req);
        crate::grpc::tran_rpc::tran::main(req,chain)
    }
}

#[tonic::async_trait]
impl Token for MyGreeter {
    async fn token_rpc(
        &self,
        request: Request<TokenRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        let ref mut chain = (*BLOCK_CHAIN).lock().unwrap();
        let req:TokenRequest = request.into_inner().clone();
        crate::grpc::token_rpc::create_token::main(req,chain)
    }
}