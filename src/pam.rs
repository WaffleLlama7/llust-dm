use pam;
use pwd;
use maildir;
use fork::{ Fork, fork };

use std::env; 
use std::process::Command;

pub fn login(username: String, password: String) {
    let service = "llust-dm";

    let pw = pwd::Passwd::from_name(&username.clone());
    init_env(&pw.unwrap().unwrap());

    let mut auth = pam::Authenticator::with_password(service).expect("Failed to init PAM client.");
    auth.get_handler().set_credentials(username, password);

    auth.authenticate().expect("Auth failed");
    auth.open_session().expect("Failed to open session.");

    if auth.authenticate().is_ok() && auth.open_session().is_ok() {
        println!("Authenticated!");

       match fork() {
            Ok(Fork::Parent(child)) => {
                println!(".xinitrc pid = {}", child);
                Command::new("/bin/bash").args(&["--login", ".xinitrc"]).spawn().expect("Window Manager failed to start");
            },
            _ => (), 
       }
    } else {
        println!("Login Failed");
    }
}

fn init_env(pw: &pwd::Passwd) {
    let mdir = maildir::Maildir::from(pw.dir.clone());
    
    env::set_var("HOME", pw.dir.clone());
    env::set_var("PWD", pw.dir.clone());
    env::set_var("SHELL", pw.shell.clone());
    env::set_var("USER", pw.name.clone());
    env::set_var("LOGNAME", pw.name.clone());
    env::set_var("PATH", "/usr/local/sbin:/usr/local/bin:/usr/bin");
    env::set_var("MAIL", mdir.path());
    env::set_var("XAUTHORITY", pw.dir.clone() + "/.xauthority");
}