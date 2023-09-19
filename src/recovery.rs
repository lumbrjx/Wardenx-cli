use crate::auth::auth;
use std::io;
use wardenx_core::encrypt::*;

use wardenx_core::password::manager::manager::Secret;
use wardenx_core::user::profile::{edit_master_password, get_user};
pub fn recovery(query: &String) {
    if query == "recover" {
        let mut recovery = String::new();
        let user = get_user();
        let recovery_key = user.unwrap().unwrap().recovery_key;
        while recovery_key != encrypt_pass(recovery.trim().to_string()) {
            recovery = "".to_string();
            println!("recovery key:");

            io::stdin()
                .read_line(&mut recovery)
                .expect("Failed to read line");
        }

        let userr = get_user();
        let username = userr.unwrap().unwrap().username;
        let mut new_password = String::new();
        println!("New password:");
        io::stdin()
            .read_line(&mut new_password)
            .expect("Failed to read line");

        if new_password.trim().len() == 0 {
            panic!("passowrd can't be NULL")
        }
        let _ = edit_master_password(&username, encrypt_pass(new_password.trim().to_string()));
        println!("master password edited!")
    }
}
