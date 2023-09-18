use std::io;
pub fn edit(query: &String, flag: Option<String>) {
    if query == "edit" {
        if flag == Some("-u".to_string()) {
            let mut username = String::new();
            println!("Add your new username");
            // Read a line of input from the user and store it in the 'input' variable
            io::stdin()
                .read_line(&mut username)
                .expect("Failed to read line");
            //password checking

            // Print the user's input
            println!("You entered: {} ", username.trim());
        } else if flag == Some("-p".to_string()) {
            let mut old_password = String::new();
            let mut new_password = String::new();
            println!("Old password:");
            // Read a line of input from the user and store it in the 'input' variable
            io::stdin()
                .read_line(&mut old_password)
                .expect("Failed to read line");
            println!("New password:");
            io::stdin()
                .read_line(&mut new_password)
                .expect("Failed to read line");
        } else if flag == Some("-k".to_string()) {
            let mut key = String::new();
            println!("new key:");
            // Read a line of input from the user and store it in the 'input' variable
            io::stdin()
                .read_line(&mut key)
                .expect("Failed to read line");
            // password checking
        } else if flag == Some("-l".to_string()) {
            let mut label = String::new();
            let mut new_password = String::new();
            println!("Add the secret you want to edit:");
            // Read a line of input from the user and store it in the 'input' variable
            io::stdin()
                .read_line(&mut label)
                .expect("Failed to read line");
            println!("New label:");
            // Read a line of input from the user and store it in the 'input' variable
            io::stdin()
                .read_line(&mut new_password)
                .expect("Failed to read line");
            // password checking
        } else if flag == Some("-s".to_string()) {
            let mut old_label = String::new();
            let mut new_label = String::new();
            println!("Add the secret you want to edit:");
            // Read a line of input from the user and store it in the 'input' variable
            io::stdin()
                .read_line(&mut old_label)
                .expect("Failed to read line");
            println!("New secret:");
            // Read a line of input from the user and store it in the 'input' variable
            io::stdin()
                .read_line(&mut new_label)
                .expect("Failed to read line");
        // password checking
        } else {
            println!("Please enter a valid flag")
        }
    }
    println!("Searching for {}", query);
}
