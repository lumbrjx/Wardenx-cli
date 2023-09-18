use std::io;
pub fn init(query: &String) {
    if query == "init" {
        let mut username = String::new();
        let mut master_password = String::new();
        let mut recovery_key = String::new();
        println!("Choose a username");
        // Read a line of input from the user and store it in the 'input' variable
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

        // Print the user's input
        println!(
            "You entered: {} {} {}",
            username.trim(),
            master_password.trim(),
            recovery_key.trim()
        );
    }
    println!("Searching for {}", query);
}
