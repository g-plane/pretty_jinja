use std::{env, fs};

fn main() {
    let file = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    match pretty_jinja::format_stmt(&file, &Default::default()) {
        Ok(output) => println!("{output}"),
        Err(err) => eprint!("{err}"),
    };
}
