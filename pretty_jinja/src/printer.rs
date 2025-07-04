use crate::{
    config::{FormatOptions, LanguageOptions, TrailingComma},
    syntax::{NodeOrToken, SyntaxKind, SyntaxNode},
};
use tiny_pretty::Doc;

pub(crate) fn format(node: &SyntaxNode, options: &FormatOptions) -> Doc<'static> {
    print_node(
        node,
        &Ctx {
            indent_width: options.layout.indent_width,
            options: &options.language,
        },
    )
}

struct Ctx<'b> {
    indent_width: usize,
    options: &'b LanguageOptions,
}

fn print_node(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    match node.kind() {
        SyntaxKind::ARG => todo!(),
        SyntaxKind::EXPR_BIN => print_expr_bin(node, ctx),
        SyntaxKind::EXPR_CALL => todo!(),
        SyntaxKind::EXPR_CONCAT => print_expr_concat(node, ctx),
        SyntaxKind::EXPR_DICT => todo!(),
        SyntaxKind::EXPR_DICT_ITEM => todo!(),
        SyntaxKind::EXPR_FILTER => print_expr_filter(node, ctx),
        SyntaxKind::EXPR_GET_ATTR => print_expr_get(node, ctx),
        SyntaxKind::EXPR_GET_ITEM => print_expr_get(node, ctx),
        SyntaxKind::EXPR_IDENT => print_expr_ident(node),
        SyntaxKind::EXPR_IF => todo!(),
        SyntaxKind::EXPR_LIST => print_expr_list(node, ctx),
        SyntaxKind::EXPR_LITERAL => print_expr_literal(node, ctx),
        SyntaxKind::EXPR_PAREN => print_expr_paren(node, ctx),
        SyntaxKind::EXPR_TEST => todo!(),
        SyntaxKind::EXPR_TUPLE => print_expr_tuple(node, ctx),
        SyntaxKind::EXPR_UNARY => todo!(),
        SyntaxKind::PARAM => todo!(),
        SyntaxKind::STMT_CALL => todo!(),
        SyntaxKind::STMT_FILTER => todo!(),
        SyntaxKind::STMT_FOR => todo!(),
        SyntaxKind::STMT_MACRO => todo!(),
        SyntaxKind::STMT_SET => todo!(),
        SyntaxKind::STMT_UNKNOWN => todo!(),
        SyntaxKind::STMT_WITH => todo!(),
        SyntaxKind::ROOT_EXPR => print_root_expr(node, ctx),
        SyntaxKind::ROOT_STMT => todo!(),
        _ => unreachable!("only syntax node is expected"),
    }
}

fn print_expr_bin(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    let doc = print_expr_with_operator(node, ctx);
    if node
        .parent()
        .is_some_and(|node| node.kind() == SyntaxKind::EXPR_BIN)
    {
        doc
    } else {
        doc.group()
    }
}

fn print_expr_concat(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_expr_with_operator(node, ctx).group()
}

fn print_expr_filter(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_expr_with_operator(node, ctx).group()
}

fn print_expr_get(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|element| element.kind() != SyntaxKind::WHITESPACE)
            .map(|element| match element {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) => Doc::text(token.text().to_string()),
            })
            .collect(),
    )
}

fn print_expr_ident(node: &SyntaxNode) -> Doc<'static> {
    node.first_token()
        .map(|token| Doc::text(token.text().to_string()))
        .unwrap_or_else(Doc::nil)
}

fn print_expr_list(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_comma_separated_with_delimiter(
        node,
        ctx,
        ctx.options.expr_list_trailing_comma,
        ctx.options.expr_list_prefer_single_line,
        ctx.options.bracket_spacing,
    )
}

fn print_expr_literal(node: &SyntaxNode, _: &Ctx) -> Doc<'static> {
    node.first_token()
        .map(|token| Doc::text(token.text().to_string()))
        .unwrap_or_else(Doc::nil)
}

fn print_expr_paren(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|element| element.kind() != SyntaxKind::WHITESPACE)
            .map(|element| match element {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) => Doc::text(token.text().to_string()),
            })
            .collect(),
    )
}

fn print_expr_tuple(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_comma_separated_with_delimiter(
        node,
        ctx,
        if node.children().count() == 1 {
            Some(TrailingComma::Always)
        } else {
            ctx.options.expr_tuple_trailing_comma
        },
        ctx.options.expr_tuple_prefer_single_line,
        ctx.options.tuple_paren_spacing,
    )
}

fn print_expr_with_operator(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|element| element.kind() != SyntaxKind::WHITESPACE)
            .map(|element| match element {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) if token.kind() == SyntaxKind::OPERATOR => {
                    let (prefix, suffix) = get_operator_space(ctx);
                    if token.kind() == SyntaxKind::OPERATOR {
                        prefix
                            .append(Doc::text(token.text().to_string()))
                            .append(suffix)
                    } else {
                        Doc::text(token.text().to_string())
                    }
                }
                NodeOrToken::Token(token) => Doc::text(token.text().to_string()),
            })
            .collect(),
    )
}

fn print_root_expr(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    node.first_child()
        .map(|child| print_node(&child, ctx))
        .unwrap_or_else(Doc::nil)
}

fn print_comma_separated_with_delimiter(
    node: &SyntaxNode,
    ctx: &Ctx,
    trailing_comma: Option<TrailingComma>,
    prefer_single_line: Option<bool>,
    delim_spacing: bool,
) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|element| element.kind() != SyntaxKind::WHITESPACE)
            .map(|element| match element {
                NodeOrToken::Node(child) => {
                    let doc = print_node(&child, ctx);
                    let doc = if node.last_child().is_some_and(|last| last == child) {
                        let comma = match trailing_comma.unwrap_or(ctx.options.trailing_comma) {
                            TrailingComma::Never => Doc::nil(),
                            TrailingComma::Always => Doc::text(","),
                            TrailingComma::OnlyMultiLine => {
                                Doc::flat_or_break(Doc::nil(), Doc::text(","))
                            }
                        };
                        doc.append(comma)
                    } else {
                        doc.append(Doc::text(",")).append(Doc::line_or_space())
                    };
                    doc.nest(ctx.indent_width)
                }
                NodeOrToken::Token(token) => {
                    if token.kind() == SyntaxKind::COMMA {
                        Doc::nil()
                    } else {
                        let doc = Doc::text(token.text().to_string());
                        if token.index() == 0 {
                            let ws = if !prefer_single_line
                                .unwrap_or(ctx.options.prefer_single_line)
                                && token.next_token().is_some_and(|token| {
                                    token.kind() == SyntaxKind::WHITESPACE
                                        && token.text().contains('\n')
                                }) {
                                Doc::hard_line()
                            } else if delim_spacing {
                                Doc::line_or_space()
                            } else {
                                Doc::line_or_nil()
                            };
                            doc.append(ws).nest(ctx.indent_width)
                        } else if node.last_token() == Some(token) {
                            let ws = if delim_spacing {
                                Doc::line_or_space()
                            } else {
                                Doc::line_or_nil()
                            };
                            ws.append(doc)
                        } else {
                            doc
                        }
                    }
                }
            })
            .collect(),
    )
    .group()
}

fn get_operator_space(ctx: &Ctx) -> (Doc<'static>, Doc<'static>) {
    use crate::config::OperatorLineBreak;
    match ctx.options.operator_linebreak {
        OperatorLineBreak::Before => (Doc::line_or_space().nest(ctx.indent_width), Doc::space()),
        OperatorLineBreak::After => (Doc::space(), Doc::line_or_space().nest(ctx.indent_width)),
    }
}
