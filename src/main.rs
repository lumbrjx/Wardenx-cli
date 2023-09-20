use std::env;
mod add;
mod del;
mod edit;
mod get;
mod help;
use del::del;
use edit::edit;
use get::get;
use help::help;
mod init;
use add::add;
use init::init;
mod auth;
mod recovery;
use recovery::recovery;
mod generate;
use generate::generate;
mod hash;
use hash::hash;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let flag = args.get(2).cloned();

    let _ = help(query);
    let _ = init(query);
    let _ = add(query);
    let _ = del(query, flag.clone());
    let _ = get(query, flag.clone());
    let _ = edit(query, flag.clone());
    let _ = recovery(query);
    let _ = generate(query);
    let _ = hash(query);
}
