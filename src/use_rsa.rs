use std::fs;
use std::str;
use std::io::{self, BufRead};

use crate::rsa;

const MESSAGE_MAX_LENGTH: usize = 257;

pub fn encrypt_file(filename:&str, destination:&str){
    let my_rsa = rsa::RSA::new();

    let mut content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    if content.len() > MESSAGE_MAX_LENGTH {
        content.split_off(MESSAGE_MAX_LENGTH);
    }

    let encrypted_content = my_rsa.encriptar(&content);
    let encrypted_content = str::from_utf8(&encrypted_content)
        .expect("Invalid UTF-8 sequence");
    let mut key = my_rsa.get_e().to_string();
    key.push_str("\n");
    key.push_str(&my_rsa.get_n().to_string());

    let mut path = String::from(destination);
    path.push_str("/encrypted_");
    path.push_str(filename);

    let mut keypath = String::from(destination);
    keypath.push_str("/key.txt");

    fs::write(path, encrypted_content)
        .expect("Something went wrong writing the file");
    fs::write(keypath, key)
        .expect("Something went wrong writing the key");
}


pub fn decrypt_file(filename:&str, keyfile:&str, destination:&str){
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let keyf = fs::File::open(keyfile)
        .expect("Something went wrong reading the keyfile");
    let buffer = io::BufReader::new(keyf);
    let mut e:u64 = 0;
    let mut n:u64 = 0;

    for (i, line) in buffer.lines().enumerate(){
        if i==0 {
            e = line.unwrap().parse::<u64>().unwrap();
        } else {
            n = line.unwrap().parse::<u64>().unwrap();
        }
    }

    let message = rsa::RSA::desencriptar_con_clave(&content, e, n);

    let mut path = String::from(destination);
    path.push_str("/plain_");
    path.push_str(filename);

    fs::write(path, message)
        .expect("Something went wrong writing the file");
}
