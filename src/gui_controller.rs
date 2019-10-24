extern crate gtk;

use gtk::prelude::*;
use crate::use_rsa;

/// Define el comportamiento de la interfaz gráfica y la inicializa
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
        let destination = manage_entry(&destination_encrypt_button,
                                    "You have to indicate a destination");
        if !destination.0 {return;}
        let destination = destination.1;

        let message = entry.get_text().unwrap().to_string();
        if message == "" {
            let filename = manage_entry(&file_chooser_button,
                                 "You have to indicate a file to encypt");
            if !filename.0 {return;}
            let filename = filename.1;
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
        let destination = manage_entry(&destination_decrypt_button,
                                   "You have to indicate a destination");
        if !destination.0 {return;}
        let destination = destination.1;

        let keyfile = manage_entry(&key_file_button,
                                   "You have to indicate a keyfile");
        if !keyfile.0 {return;}
        let keyfile = keyfile.1;

        let filename = manage_entry(&decrypted_file_button,
                                    "You have to indicate a file to decrypt");
        if !filename.0 {return;}
        let filename = filename.1;

        use_rsa::decrypt_file(&filename, &keyfile, &destination);
    });

    window.show_all();
    gtk::main();
}

///Maneja la información proveida por un filechooser button. Si el
/// botón devuelve None regresa una tupla donde la primera entrada es
/// false, para cualquier otro caso regresa una tupla donde la primera
/// entrada es true y la segunda es una cadena con la dirección del
/// archivo seleccionado en 'filechooser'
fn manage_entry(filechooser:&gtk::FileChooserButton,
                message:&str) -> (bool, String){
    let fc = filechooser.get_uri();
    if fc.is_none() {
        println!("{}", message);
        return (false, String::from(""));
    }
    let fc = fc.unwrap().to_string().split_off(7);
    (true, fc)
}
