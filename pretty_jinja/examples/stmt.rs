use std::{env, fs};

fn main() {
    let file = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    match pretty_jinja::parse_stmt(&file) {
        Ok(tree) => print!("{tree:#?}"),
        Err(err) => eprint!("{err}"),
    };
}
