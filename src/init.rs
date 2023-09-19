use std::io;
use wardenx_core::database::warden_db::*;
use wardenx_core::encrypt::*;
use wardenx_core::user::profile::*;
pub fn init(query: &String) {
    if query == "init" {
        let _ = create_db_tables();
        let profile_exists = get_profile_table_length();
        if profile_exists.ok() >= Some(1) {
            panic!("You already have an account!")
        }
        let mut username = String::new();
        let mut master_password = String::new();
        let mut recovery_key = String::new();
        println!("Choose a username");
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");
        println!("Add your master password");
        io::stdin()
            .read_line(&mut master_password)
            .expect("Failed to read line");
        println!("Add a recovery key *DONT FORGET IT*");
        io::stdin()
            .read_line(&mut recovery_key)
            .expect("Failed to read line");

        if username.trim().len() == 0
            || master_password.trim().len() == 0
            || recovery_key.trim().len() == 0
        {
            panic!("username, master passowrd, recovery key. Can't be NULL")
        }
        let user = User {
            username: username.trim().to_string(),
            master_password: encrypt_pass(master_password.trim().to_string()),
            recovery_key: encrypt_pass(recovery_key.trim().to_string()),
        };
        match user.create_user() {
            Ok(t) => t,
            Err(err) => println!("{}", err),
        };
    }
}
