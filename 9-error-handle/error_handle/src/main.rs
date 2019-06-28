use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) =>  panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file {:?}", other_error)
        }
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => s,
        Err(e) => panic!("Error to read file {:?}", e),
    };

    match read_username_from_file() {
        Ok(s1) => println!("Username {}", s1),
        Err(e) => panic!("Error {:?}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
