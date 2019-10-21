extern crate gtk;

use gtk::prelude::*;
use crate::rsa;

pub fn build_gui() {

    let _rsa_system = rsa::RSA::new();

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("gui.xml");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window").unwrap();
	window.connect_delete_event(|_,_| {gtk::main_quit(); Inhibit(false) });
    window.set_title("RSA system");

    let encrypt_button: gtk::Button = builder
        .get_object("encrypt_button").unwrap();
    encrypt_button.connect_clicked(move |_| {
        println!("encrypt");
    });

    let decrypt_button: gtk::Button = builder
        .get_object("decrypt_button").unwrap();
    decrypt_button.connect_clicked(move |_| {
        println!("decrypt");
    });

    window.show_all();
    gtk::main();
}

