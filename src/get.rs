use std::io;
pub fn get(query: &String, flag: Option<String>) {
    if query == "get" {
        if flag == Some("-s".to_string()) {
            let mut label = String::new();
            println!("What's your secret name?");
            // Read a line of input from the user and store it in the 'input' variable
            io::stdin()
                .read_line(&mut label)
                .expect("Failed to read line");

            // Print the user's input
            println!("You entered: {} ", label.trim());
        } else if flag == Some("-h".to_string()) {
            println!("history retrieved")
        } else if flag == Some("-a".to_string()) {
            println!("all secrets retrieved")
        } else {
            println!("Please enter a valid flag")
        }
    }
    println!("Searching for {}", query);
}
