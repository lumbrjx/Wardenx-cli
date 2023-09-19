use std::io;

use wardenx_core::{password::manager::manager::*, profile::history::history::delete_all_records};

use crate::auth::auth;
pub fn del(query: &String, flag: Option<String>) {
    if query == "del" {
        if flag == Some("-s".to_string()) {
            let mut label = String::new();
            println!("What's your secret name?");
            // Read a line of input from the user and store it in the 'input' variable
            io::stdin()
                .read_line(&mut label)
                .expect("Failed to read line");
            let _ = auth();
            let _ = delete_secret(label.trim().to_string());
            println!("secret deleted!")

            // Print the user's input
        } else if flag == Some("-h".to_string()) {
            let _ = auth();
            let _ = delete_all_records();
            println!("history is cleared")
        } else {
            println!("Please enter a valid flag")
        }
    }
}
