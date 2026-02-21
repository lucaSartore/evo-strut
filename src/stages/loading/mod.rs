use std::fs::OpenOptions;

pub fn read(name: &str) {
    let mut file = OpenOptions::new()
        .read(true)
        .open(name)
        .unwrap();
    let stl = stl_io::read_stl(&mut file)
        .unwrap();
    println!("{:?}", stl)
}
