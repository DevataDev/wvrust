use std::io::Result;
use prost_build::Config;

fn main() -> Result<()> {
    // prost_build::compile_protos(&["src/libs/license_protocol.proto"], &["src/"])?;
    Config::new().out_dir("src").compile_protos(&["src/libs/license_protocol.proto"], &["src/"])?;
    Ok(())
}