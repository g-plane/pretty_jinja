use crate::config::FormatOptions;

pub mod config;
mod parser;
mod printer;
mod syntax;

pub fn format_expr(code: &str, options: &FormatOptions) -> Result<String, String> {
    let node = match crate::parser::parse_expr(code) {
        Ok(node) => node,
        Err(err) => return Err(err.to_string()),
    };
    Ok(tiny_pretty::print(
        &printer::format(&node, options),
        &options.layout.clone().into(),
    ))
}

pub fn format_stmt(code: &str, options: &FormatOptions) -> Result<String, String> {
    let node = match crate::parser::parse_stmt(code) {
        Ok(node) => node,
        Err(err) => return Err(err.to_string()),
    };
    Ok(tiny_pretty::print(
        &printer::format(&node, options),
        &options.layout.clone().into(),
    ))
}
