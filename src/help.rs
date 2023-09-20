pub fn help(query: &String) {
    if query == "help" {
        println!("Wardenx CLI commands guide:");
        println!(" - help : CLI command guide");
        println!(" - init : create a databse with new profile");
        println!(" - generate : generates a hashed password");
        println!(" - hash : takes an string as an input and hash it");
        println!(" - recover : password recovery");
        println!(" - add : adds a new secret to the database");
        println!(" - del :");
        println!("   - s (secret) : delete a secret");
        println!("   - h (history) : delete the history");
        println!(" - get :");
        println!("   - h (history) : show the logs");
        println!("   - s (secret) : show a specified secret");
        println!("   - a (all) : show a list with all secrets");
        println!(" - edit :");
        println!("   - u (username) : edits the username");
        println!("   - p (password) : edits the master password");
        println!("   - k (key) : edits the recovery key");
        println!("   - l (label) : edits the secret label");
        println!("   - s (secret) : edits the secret password");
    }
}
// init => create tables, and username, password, recovery_key
// add *label* *passowrd*
// del -s *label*
// del -h
// edit -u *new username*
// edit -p *old password* *new passowrd*
// edit -k *password* *new key*
// edit -l *old label* *new label*
// edit -s *label* *new passowrd*
// get -h => shows logs
// get -s *label*
// get -a
// help
// recover
// generate
// hash
