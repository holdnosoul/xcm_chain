fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/greeter.proto")?;
    tonic_build::compile_protos("proto/tran.proto")?;
    tonic_build::compile_protos("proto/token.proto")?;
    Ok(())
}
