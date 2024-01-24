use wardenx_core::{
    encrypt_pass,
    password::manager::manager::edit_secret_label,
    password::manager::manager::edit_secret_password,
    user::profile::{edit_master_password, edit_recovery_key, edit_username, get_user},
};

use crate::auth::auth;
use std::io;

pub fn edit(query: &String, flag: Option<String>) {
    if query == "edit" {
        if flag == Some("-u".to_string()) {
            let user = get_user();
            let old_username = user.unwrap().unwrap().username;
            let mut username = String::new();
            println!("Add your new username");
            io::stdin()
                .read_line(&mut username)
                .expect("Failed to read line");
            if username.trim().len() == 0 {
                panic!("username can't be NULL")
            }
            let _ = auth();
            let _ = edit_username(&old_username, username.trim().to_string());
            println!("user edited!")
        } else if flag == Some("-p".to_string()) {
            let user = get_user();
            let username = user.unwrap().unwrap().username;
            let mut new_password = String::new();
            let _ = auth();
            println!("New password:");
            io::stdin()
                .read_line(&mut new_password)
                .expect("Failed to read line");

            if new_password.trim().len() == 0 {
                panic!("passowrd can't be NULL")
            }
            let _ = edit_master_password(
                &username,
                encrypt_pass(
                    new_password.trim().to_string(),
                    new_password.trim().to_string(),
                ),
            );
            println!("master password edited!")
        } else if flag == Some("-k".to_string()) {
            let user = get_user();
            let username = user.unwrap().unwrap().username;
            let mut key = String::new();
            let _ = auth();
            println!("new key:");
            io::stdin()
                .read_line(&mut key)
                .expect("Failed to read line");
            if key.trim().len() == 0 {
                panic!("recovery key can't be NULL")
            }
            let _ = edit_recovery_key(
                &username,
                encrypt_pass(key.trim().to_string(), key.trim().to_string()),
            );
            println!("recovery key edited!")
        } else if flag == Some("-l".to_string()) {
            let mut label = String::new();
            let mut new_label = String::new();
            println!("Add the secret you want to edit:");
            io::stdin()
                .read_line(&mut label)
                .expect("Failed to read line");
            println!("New label:");
            io::stdin()
                .read_line(&mut new_label)
                .expect("Failed to read line");
            if new_label.trim().len() == 0 {
                panic!("secret must have a name")
            }
            let _ = auth();
            let _ = edit_secret_label(&label.trim().to_string(), new_label.trim().to_string());
            println!("secret edited!")
        } else if flag == Some("-s".to_string()) {
            let mut old_label = String::new();
            let mut new_label = String::new();
            println!("Add the secret you want to edit:");
            io::stdin()
                .read_line(&mut old_label)
                .expect("Failed to read line");
            println!("New secret:");
            io::stdin()
                .read_line(&mut new_label)
                .expect("Failed to read line");
            if new_label.trim().len() == 0 {
                panic!("empty secrets are not valid")
            }
            let _ = auth();
            let _ = edit_secret_password(
                &old_label.trim().to_string(),
                encrypt_pass(new_label.trim().to_string(), new_label.trim().to_string()),
            );
            println!("secret edited!")
        } else {
            println!("Please enter a valid flag")
        }
    }
}
