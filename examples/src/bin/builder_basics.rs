//! # Builder Basics Sample
//!
//! This sample demonstrates how to use the builder with an imported UI file

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button, MessageDialog, ResponseType};

use std::env::args;

fn build_ui(application: &Application) {
    let ui_src = include_str!("ui/builder_basics.ui");
    let builder = Builder::from_string(ui_src);

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let bigbutton: Button = builder.get_object("button").expect("Couldn't get button");
    let dialog: MessageDialog = builder
        .get_object("messagedialog")
        .expect("Couldn't get messagedialog");

    dialog.connect_response(move |d: &MessageDialog, _: ResponseType| {
        d.hide();
    });

    bigbutton.connect_clicked(move |_| {
        dialog.show();
    });

    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
