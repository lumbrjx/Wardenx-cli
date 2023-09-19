use crate::auth::auth;
use std::io;
use wardenx_core::profile::history::history::get_history;
use wardenx_core::{
    decrypt_pass,
    password::manager::manager::{get_all_passwords, get_password},
};

pub fn get(query: &String, flag: Option<String>) {
    if query == "get" {
        if flag == Some("-s".to_string()) {
            let mut label = String::new();
            println!("What's your secret name?");
            io::stdin()
                .read_line(&mut label)
                .expect("Failed to read line");
            let _ = auth();
            let secret = get_password(label.trim().to_string());
            match secret {
                Ok(None) => println!("no secret found!"),
                Ok(t) => println!(
                    "secret name: {}, secert: {:?}",
                    label.clone().trim(),
                    decrypt_pass(t.unwrap().password)
                ),
                Err(err) => println!("{}", err),
            }
        } else if flag == Some("-h".to_string()) {
            let records = get_history();
            for secret in records.unwrap().iter() {
                println!("{} at {}", &secret.record, secret.date_time)
            }
        } else if flag == Some("-a".to_string()) {
            let labels = get_all_passwords();
            for secret in labels.unwrap().iter() {
                println!("{}", secret.label)
            }
        } else {
            println!("Please enter a valid flag")
        }
    }
}
