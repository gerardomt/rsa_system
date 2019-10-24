use std::fs;
use std::str;
use std::io::{self, BufRead};

use crate::rsa;

const MESSAGE_MAX_LENGTH: usize = 257;

/// Cifra el contenido del archivo indicado en 'filename' y crea dos
/// nuevos archivos en el directorio indicado en 'destination'; uno
/// contiene el mensaje encriptado y otro la clave que se usó para
/// encriptarlo.  Sólo son cifrados los primeros 'MESSAGE_MAX_LENGTH'
/// caracteres en 'filename' los demás son ignorados
pub fn encrypt_file(filename:&str, destination:&str){
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    encrypt_message(&content, destination);
}

/// Cifra la cadena 'message' y crea dos nuevos archivos en el
/// directorio indicado en 'destination'; uno contiene el mensaje
/// cifrado y otro, la clave que se usó para encriptarlo.  Sólo son
/// cifrados los primeros 'MESSAGE_MAX_LENGTH' caracteres en 'message'
/// los demás son ignorados
pub fn encrypt_message(message:&str, destination:&str){
    let my_rsa = rsa::RSA::new();

    let mut message = String::from(message);
    if message.len() > MESSAGE_MAX_LENGTH {
        message.split_off(MESSAGE_MAX_LENGTH);
    }

    let encrypted_content = my_rsa.encriptar(&message);
    let encrypted_content = str::from_utf8(&encrypted_content)
        .expect("Invalid UTF-8 sequence");
    let mut key = my_rsa.get_e().to_string();
    key.push_str("\n");
    key.push_str(&my_rsa.get_n().to_string());

    let mut path = String::from(destination);
    path.push_str("/encrypted_file.txt");

    let mut keypath = String::from(destination);
    keypath.push_str("/key.txt");

    fs::write(path, encrypted_content)
        .expect("Something went wrong writing the file");
    fs::write(keypath, key)
        .expect("Something went wrong writing the key");
}

/// Descifra el contenido del archivo indicado en 'filename'
/// utilizando la clave dentro del archivo 'keyfile' y guarda el texto
/// plano en un nuevo archivo dentro del directorio indicado en
/// 'destination'
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
    path.push_str("/plain_text.txt");

    fs::write(path, message)
        .expect("Something went wrong writing the fileoe");
}
