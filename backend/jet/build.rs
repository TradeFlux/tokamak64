use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    // Path to schemas directory (two levels up from backend/jet)
    let schema_dir = PathBuf::from(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("schemas");

    // Ensure schema directory exists
    if !schema_dir.exists() {
        panic!("Schema directory not found: {}", schema_dir.display());
    }

    // Schema files to generate (and their dependencies)
    let schemas = ["game.fbs", "player.fbs", "api.fbs"];

    // Tell cargo to rerun if any schema file changes
    println!(
        "cargo:rerun-if-changed={}",
        schema_dir.join("types.fbs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        schema_dir.join("board.fbs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        schema_dir.join("api.fbs").display()
    );
    for schema in &schemas {
        let schema_path = schema_dir.join(schema);
        println!("cargo:rerun-if-changed={}", schema_path.display());
    }

    // Generate code for each top-level schema
    for schema in &schemas {
        let schema_path = schema_dir.join(schema);
        let status = Command::new("flatc")
            .arg("--rust")
            .arg("-o")
            .arg(&out_dir)
            .arg("--gen-all")
            .arg("--filename-suffix")
            .arg("")
            .arg(&schema_path)
            .current_dir(&schema_dir)
            .status()
            .expect("Failed to execute flatc. Is flatbuffers installed?");

        if !status.success() {
            panic!("flatc failed for {} with status: {}", schema, status);
        }
    }

    println!("cargo:warning=Generated FlatBuffers code in {}", out_dir);
}
