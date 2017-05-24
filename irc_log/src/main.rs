use std::fs;

fn main() {
    let paths = fs::read_dir("./log").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        println!("{}", path.to_string_lossy().to_string());
    }
}
