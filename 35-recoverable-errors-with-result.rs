use std::fs::File;

fn main() {
    let f = File::open("hello.text");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}",error),
    };
}