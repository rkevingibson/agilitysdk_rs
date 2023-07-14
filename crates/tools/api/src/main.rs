use std::{fs, path::PathBuf, process::Command};
use windows_metadata::reader::File;

fn main() {
    let files = File::with_default(&[".windows/winmd/Microsoft.AgilitySdk.winmd"]).unwrap();

    let output_path = PathBuf::from("src/d3d12.rs");
    if output_path.exists() {
        fs::remove_file(&output_path).unwrap();
    }

    fs::write(
        &output_path,
        windows_bindgen::component("Microsoft.AgilitySdk", &files),
    )
    .unwrap();

    let mut child = Command::new("rustfmt")
        .args([&output_path])
        .spawn()
        .expect("Failed to start rustfmt");

    child.wait().expect("rustfmt failed");
}
