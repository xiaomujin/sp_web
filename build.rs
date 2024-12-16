use std::fs;
use std::io;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "src/pb";
    let protoc_path = "resource/protoc-29.1";

    let mut files = Vec::new();
    recurse_dir(&mut files, &proto_dir)?;

    if !Path::new(&protoc_path).exists() {
        return Err(format!("protoc executable not found at {}", protoc_path).into());
    }

    prost_build::Config::new()
        .protoc_executable(&protoc_path)
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
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
