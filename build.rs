fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tonic_build::compile_protos("proto/echo.proto");
    Ok(())
}