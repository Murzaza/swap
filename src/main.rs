use std::fs::File;
use std::io::prelude::*;
use std::env;

fn read_file(filename: String) -> Vec<u8> {

    let mut file = match File::open(filename.clone()) {
        Err(reason) =>  panic!("Unable to open file '{}': {}", filename, reason.to_string()),
        Ok(file) => file 
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Err(reason) => panic!("Unable to read file '{}': {}", filename, reason.to_string()),
        Ok(_) => {}
    };

    buffer
}

fn write_file(filename: String, data: Vec<u8>) {

    let mut file = match File::create(filename.clone()) {
        Err(reason) => panic!("Unable to open file '{}': {}", filename, reason.to_string()),
        Ok(file) => file
    }; 

    match file.write_all(data.as_ref()) {
        Err(reason) => panic!("Unable to write file '{}': {}", filename, reason.to_string()),
        Ok(_) => {}
    };
}

fn print_usage() {
    println!("Usage:\n\tswap <file-path> <file-path>");
}

fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        panic!("Not all required inputs met");
    }

    args
}

fn main() {

    let args = get_args();

    let path_1 = args.get(1).expect("Unable to retrieve first argument");
    let path_2 = args.get(2).expect("Unable to retrieve second argument");

    let data_1 = read_file(path_1.clone());
    let data_2 = read_file(path_2.clone());
    write_file(path_2.clone(), data_1);
    write_file(path_1.clone(), data_2);
}
