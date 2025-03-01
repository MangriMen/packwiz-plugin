use std::fs;

pub fn clean() {
    let _ = fs::remove_dir_all("target/bundle");
    println!("Build artifacts are deleted");
}
