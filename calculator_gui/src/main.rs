extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Hello, GTK!");
    window.set_default_size(350, 70);

    let button = Button::new_with_label("Click me!");
    window.add(&button);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    window.show_all();

    gtk::main();
}