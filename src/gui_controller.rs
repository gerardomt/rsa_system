extern crate gtk;

use gtk::prelude::*;
use crate::use_rsa;

pub fn build_gui() {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("gui.xml");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window").unwrap();
        window.connect_delete_event(|_,_| {gtk::main_quit(); Inhibit(false) });
    window.set_title("RSA system");

    let entry: gtk::Entry = builder
        .get_object("message_entry").unwrap();
    let file_chooser_button: gtk::FileChooserButton = builder
        .get_object("select_message_button").unwrap();
    let destination_encrypt_button: gtk::FileChooserButton = builder
        .get_object("dest_to_encrypt_button").unwrap();
    let encrypt_button: gtk::Button = builder
        .get_object("encrypt_button").unwrap();
    encrypt_button.connect_clicked(move |_| {
        println!("encrypt");
        let destination = destination_encrypt_button.get_uri();
        if destination.is_none() {
            println!("You have to indicate a destination");
            return;
        }
        let destination = destination.unwrap().to_string().split_off(7);

        let message = entry.get_text().unwrap().to_string();
        if message == "" {
            let filename = file_chooser_button.get_uri();
            if filename.is_none() {
                println!("You have to indicate a file to encrypt");
                return;
            }
            let filename = filename.unwrap().to_string().split_off(7);
            use_rsa::encrypt_file(&filename, &destination);
        } else {
            use_rsa::encrypt_message(&message, &destination);
        }
    });

    let decrypted_file_button: gtk::FileChooserButton = builder
        .get_object("select_cipher_file_button").unwrap();
    let key_file_button: gtk::FileChooserButton = builder
        .get_object("select_key_file_button").unwrap();
    let destination_decrypt_button: gtk::FileChooserButton = builder
        .get_object("dest_to_decrypt_button").unwrap();
    let decrypt_button: gtk::Button = builder
        .get_object("decrypt_button").unwrap();
    decrypt_button.connect_clicked(move |_| {
        println!("decrypt");
        let destination = destination_decrypt_button.get_uri();
        if destination.is_none() {
            println!("You have to indicate a destination");
            return;
        }
        let destination = destination.unwrap().to_string().split_off(7);

        let keyfile = key_file_button.get_uri();
        if keyfile.is_none() {
            println!("You have to indicate a keyfile");
            return;
        }
        let keyfile = keyfile.unwrap().to_string().split_off(7);

        let filename = decrypted_file_button.get_uri();
        if filename.is_none() {
            println!("You have to indicate a file to decrypt");
            return;
        }
        let filename = filename.unwrap().to_string().split_off(7);

        use_rsa::decrypt_file(&filename, &keyfile, &destination);
    });

    window.show_all();
    gtk::main();
}
