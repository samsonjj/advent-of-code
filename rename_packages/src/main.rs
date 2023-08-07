fn main() {
    println!("Hello, world!");
    let dirs = std::fs::read_dir("./").unwrap();
    for dir in dirs.into_iter() {
        let dir = dir.unwrap();
        let sub_dirs = std::fs::read_dir(dir.path()).unwrap();
        for sub_dir in sub_dirs.into_iter() {
            let sub_dir = sub_dir.unwrap();
            let entries = std::fs::read_dir(sub_dir.path()).unwrap();
            for entry in entries.into_iter() {
                let entry = entry.unwrap();
                if !entry.file_type().unwrap().is_file() {
                    continue;
                }
                if entry.file_name() != "Cargo.toml" {
                    continue;
                }
                let contents = std::fs::read_to_string(entry.path()).unwrap();
            }
        }
    }
}
