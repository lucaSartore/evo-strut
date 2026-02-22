use std::fs::OpenOptions;
use stl_io::IndexedMesh;
use anyhow::Result;

pub fn read(name: &str) -> Result<IndexedMesh> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(name)
        .unwrap();
    Ok(stl_io::read_stl(&mut file)?)
}
