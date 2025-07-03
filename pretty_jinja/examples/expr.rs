use pretty_jinja::config::FormatOptions;
use std::{env, fs, io};

fn main() -> anyhow::Result<()> {
    let code = fs::read_to_string(env::args().nth(1).unwrap())?;
    let options = match fs::read("config.json") {
        Ok(file) => serde_json::from_reader(&*file)?,
        Err(error) => {
            if error.kind() == io::ErrorKind::NotFound {
                FormatOptions::default()
            } else {
                return Err(error.into());
            }
        }
    };

    match pretty_jinja::format_expr(&code, &options) {
        Ok(output) => println!("{output}"),
        Err(err) => eprint!("{err}"),
    };
    Ok(())
}
