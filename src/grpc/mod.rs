pub mod get_rpc;
pub mod grpc;
pub mod token_rpc;
pub mod tran_rpc;

pub mod greeter { tonic::include_proto!("greeter"); }
pub mod tran { tonic::include_proto!("tran"); }
pub mod token { tonic::include_proto!("token"); }

