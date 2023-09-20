use wardenx_core::generate_random_string;

pub fn generate(query: &String) {
    if query == "generate" {
        let string = generate_random_string();
        println!("{}", string)
    }
}
