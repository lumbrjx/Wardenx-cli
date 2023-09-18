use std::io;
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

        // Print the user's input
        println!("You entered: {} {} ", label.trim(), secret.trim(),);
    }
    println!("Searching for {}", query);
}
