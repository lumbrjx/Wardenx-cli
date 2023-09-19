// check password and username before or only password
use std::io;
use wardenx_core::encrypt::*;
pub use wardenx_core::user::profile::*;
pub fn auth() {
    let mut password = String::new();
    let user = get_user();
    let master_pass = user.unwrap().unwrap().master_password;
    while master_pass != encrypt_pass(password.trim().to_string()) {
        println!("the enterd pass {}", password.clone());
        password = "".to_string();
        println!("master password:");

        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");
    }
}
