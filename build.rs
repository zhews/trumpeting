fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/key_distribution/key_distribution.proto")?;
    Ok(())
}
