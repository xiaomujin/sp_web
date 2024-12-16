// use std::path::{Path, PathBuf};
//
// fn recurse_dir(v: &mut Vec<PathBuf>, dir: impl AsRef<Path>) {
//     for entry in
//         std::fs::read_dir(&dir).unwrap_or_else(|_| panic!("Unable to read dir: {:?}", dir.as_ref()))
//     {
//         let path = entry.expect("Unable to get directory").path();
//         if path.is_dir() {
//             recurse_dir(v, path);
//         } else if let Some(true) = path.extension().map(|v| v == "proto") {
//             v.push(path);
//         }
//     }
// }
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut files = Vec::new();
//     recurse_dir(&mut files, "src/pb");
//     prost_build::Config::new()
//         // .out_dir("src/pb")
//         .protoc_executable("resource/protoc-29.1")
//         .compile_protos(&files, &["src/pb"])?;
//     Ok(())
// }

use std::env;
use std::fs;
use std::path::Path;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = env::var("PROTO_DIR").unwrap_or_else(|_| "src/pb".to_string());
    let protoc_path = env::var("PROTOC_PATH").unwrap_or_else(|_| "resource/protoc-29.1".to_string());

    let mut files = Vec::new();
    recurse_dir(&mut files, &proto_dir)?;

    if !Path::new(&protoc_path).exists() {
        return Err(format!("protoc executable not found at {}", protoc_path).into());
    }

    prost_build::Config::new()
        .protoc_executable(&protoc_path)
        .compile_protos(&files, &[&proto_dir])?;

    Ok(())
}

fn recurse_dir(files: &mut Vec<String>, dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            recurse_dir(files, &path.display().to_string())?;
        } else if path.extension().map_or(false, |ext| ext == "proto") {
            files.push(path.display().to_string());
        }
    }
    Ok(())
}

