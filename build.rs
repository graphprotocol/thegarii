use std::io::Result;
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=proto");
    tonic_build::configure()
        .out_dir("src/protobuf")
        .format(true)
        .compile(&["proto/block.proto"], &["proto"])
}
