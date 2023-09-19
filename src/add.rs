use crate::auth::auth;
use std::io;
use wardenx_core::encrypt::*;

use wardenx_core::password::manager::manager::Secret;
pub fn add(query: &String) {
    if query == "add" {
        let mut label = String::new();
        let mut secret = String::new();
        println!("What's your secret name?");
        // Read a line of input from the user and store it in the 'input' variable
        io::stdin()
            .read_line(&mut label)
            .expect("Failed to read line");
        println!("Add your secret");
        io::stdin()
            .read_line(&mut secret)
            .expect("Failed to read line");
        if label.trim().len() == 0 || secret.trim().len() == 0 {
            panic!("label, secret. Can't be NULL")
        }
        let _ = auth();
        // Print the user's input
        let a_secret = Secret {
            label: label.trim().to_string(),
            password: encrypt_pass(secret.trim().to_string()),
        };
        match a_secret.create_secret() {
            Ok(t) => t,
            Err(err) => println!("{}", err),
        };
    }
}
