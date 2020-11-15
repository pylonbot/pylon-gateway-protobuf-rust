#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! tonic-build = { version = "0.3" }
//! walkdir = { version = "2.3.1" }
//! ```

const PROTO_IN_DIR: &'static str = "./proto";
const STUBS_OUT_DIR: &'static str = "./src/stubs";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect .proto files from the proto dir.
    let proto_files: Vec<_> = walkdir::WalkDir::new(PROTO_IN_DIR)
        .into_iter()
        .filter_map(|e| {
            let file_name = e.as_ref().unwrap().path().to_str().unwrap();
            if file_name.ends_with(".proto") && !file_name.contains(&"/google/") {
                return Some(file_name.to_owned());
            }
            None
        })
        .collect();

    // Generate code
    tonic_build::configure()
        .format(true)
        .out_dir(STUBS_OUT_DIR)
        .compile(proto_files.as_slice(), &[PROTO_IN_DIR.to_string()])?;

    Ok(())
}
