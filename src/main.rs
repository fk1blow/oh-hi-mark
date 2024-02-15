use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./src/test-file.md").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    println!("{}", contents);
}
