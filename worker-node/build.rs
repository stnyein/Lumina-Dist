fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../core-engine/proto/inference.proto")?;
    Ok(())
}
