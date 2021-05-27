mod block_chain;
mod grpc;

use grpc::greeter;
use grpc::tran;
use grpc::token;
use tonic::{transport::Server, Request, Status, metadata::MetadataValue};
use grpc::grpc::MyGreeter;

use greeter::greeter_server::GreeterServer;
use token::token_server::TokenServer;
use tran::tran_server::TranServer;

use block_chain::BLOCK_CHAIN;

// Runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        let ref mut _chain = (*BLOCK_CHAIN).lock()?;
    }
    let addr = "127.0.0.1:30000".parse()?;
    let greeter = MyGreeter::default();

    let svc_json_rpc = GreeterServer::with_interceptor(greeter, check_auth1);
    let svc_tran = TranServer::with_interceptor(greeter, check_auth2);
    let svc_token = TokenServer::with_interceptor(greeter, check_auth3);

    println!("Starting gRPC Server...at {:?}",addr);
    Server::builder()
        .add_service(svc_json_rpc)
        .add_service(svc_tran)
        .add_service(svc_token)
        .serve(addr)
        .await?;

    Ok(())
}

fn check_auth1(req: Request<()>) -> Result<Request<()>, Status> {
    let _token = MetadataValue::from_str("token1").unwrap();

    match req.metadata().get("token") {
        // Some(t) if token == t => Ok(req),
        // _ => Err(Status::unauthenticated("No valid auth token")),
        _ => Ok(req),
    }
}

fn check_auth2(req: Request<()>) -> Result<Request<()>, Status> {
    let _token = MetadataValue::from_str("token1").unwrap();

    match req.metadata().get("token") {
        // Some(t) if token == t => Ok(req),
        // _ => Err(Status::unauthenticated("No valid auth token")),
        _ => Ok(req),
    }
}

fn check_auth3(req: Request<()>) -> Result<Request<()>, Status> {
    let _token = MetadataValue::from_str("token1").unwrap();

    match req.metadata().get("token") {
        // Some(t) if token == t => Ok(req),
        // _ => Err(Status::unauthenticated("No valid auth token")),
        _ => Ok(req),
    }
}

