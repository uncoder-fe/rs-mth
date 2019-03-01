// #[derive(Debug)]
use std::env;
mod blog;

fn main() {
    let args: Vec<String> = env::args().collect();
    let c_name = &args[1];
    match c_name as &str {
        "init" => {
            let p_name = &args[2];
            blog::init(p_name.to_string());
        }
        "new" => {
            let f_name = &args[2];
            blog::new(f_name.to_string());
        }
        "build" => blog::build(),
        "server" => blog::server(),
        _ => println!("something else!"),
    }
}
