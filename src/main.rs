use gtk::{ Builder, Window, Entry };
use gtk::gdk;
use gtk::prelude::*;
use gtk::gdk::Screen;
use gtk::gdk::keys::Key;

use std::path::Path;
use std::process::Command;
use std::env;

use fork::{ Fork, fork };

mod pam;

const WINDOW_ID: &str = "window";
const USERNAME_ID: &str = "username_text_entry";
const PASSWORD_ID: &str = "password_text_entry";
const DISPLAY: &str = ":1";
const VT: &str = "vt01";

#[derive(Clone)]
struct UI {
    window: Window,
    user_text_field: Entry,
    pass_text_field: Entry,
}

impl UI {
    pub fn new(builder: Builder) -> Self {
        UI {
            window: builder.object(WINDOW_ID).expect("Could not get object `window` from builder"),
            user_text_field : builder.object(USERNAME_ID).expect("Could not get object `user_text_field` from builder"),
            pass_text_field : builder.object(PASSWORD_ID).expect("Could not get object `pass_text_field` from builder"),
        }
    }

    pub fn get_username(&self) -> String{
        self.user_text_field.text().to_string()
    }

    pub fn get_password(&self) -> String{
        self.pass_text_field.text().to_string()
    }
}

fn login_func(window: &Window, event: &gdk::EventKey, username: &String, password: &String) { 
    if event.keyval() == Key::from(65293) {        
        pam::login(username.to_string(), password.to_string());
        window.hide();
    }
}

fn start_x_server(display: &str, vt: &str) {
    match fork() {
        Ok(Fork::Parent(child)) => {
            println!("X server pid = {}", child);
            Command::new("X").args(&[display, vt]).spawn().expect("Failed to start X server");
        },
        _ => {}
    }
}

fn main() {    
    env::set_var("DISPLAY", DISPLAY);
    start_x_server(DISPLAY, VT);
    Command::new("xhost").arg("+").spawn().expect("Xhost failed");
    gtk::init().unwrap();

    if !Path::new("/proc/self/exe").exists() {
        eprintln!("Error: could not find binary");
        std::process::exit(1);
    }

    let ui: UI = UI::new(Builder::from_string(include_str!("../gui.ui")));

    let (width, height) = (Screen::screen_width(), Screen::screen_height());
    ui.window.set_default_size(width, height);
    ui.window.show();
    
    let ui2 = ui.clone();

    ui.window.connect_key_release_event(move |w, e| {
        let val = (*e).keyval();
        if val == Key::from(65293){
            let user = &ui2.get_username().clone();
            let pass = &ui2.get_password().clone();
            login_func(w, e, &user, &pass);
        }
        Inhibit(true)
    });

    ui.window.connect_delete_event(|_, _| {gtk::main_quit(); Inhibit(true)});
    gtk::main();
}
