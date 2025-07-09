use crate::config::FormatOptions;

pub mod config;
mod parser;
mod printer;
mod syntax;

/// Format Jinja expression which is generally from Jinja interpolation.
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

/// Format Jinja statement which is generally from Jinja block.
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
