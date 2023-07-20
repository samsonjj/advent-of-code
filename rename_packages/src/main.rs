fn main() {
    println!("Hello, world!");
    let dirs = std::fs::read_dir("./").unwrap();
    dbg!(&dirs.collect::<Vec<_>>());
}
