use std::env;
mod add;
mod del;
mod get;
use del::del;
use get::get;
mod init;
use add::add;
use init::init;
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

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let flag = args.get(2).cloned();

    let _ = init(query);
    let _ = add(query);
    let _ = del(query, flag.clone());
    let _ = get(query, flag.clone());
    println!("your flag {:?}", flag)
}
