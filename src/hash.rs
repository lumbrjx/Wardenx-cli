use std::io;
use wardenx_core::hash_string;

pub fn hash(query: &String) {
    if query == "hash" {
        let mut label = String::new();
        println!("Add your string here:");
        io::stdin()
            .read_line(&mut label)
            .expect("Failed to read line");
        let string = hash_string(&label);
        println!("hashed string : {}", string)
    }
}
