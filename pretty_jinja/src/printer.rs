use crate::{
    config::{FormatOptions, LanguageOptions, TrailingComma},
    syntax::{NodeOrToken, SyntaxKind, SyntaxNode},
};
use rowan::{Direction, ast::support};
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
        SyntaxKind::ARG => print_arg(node, ctx),
        SyntaxKind::EXPR_BIN => print_expr_bin(node, ctx),
        SyntaxKind::EXPR_CALL => print_expr_call(node, ctx),
        SyntaxKind::EXPR_CONCAT => print_expr_concat(node, ctx),
        SyntaxKind::EXPR_DICT => print_expr_dict(node, ctx),
        SyntaxKind::EXPR_DICT_ITEM => print_expr_dict_item(node, ctx),
        SyntaxKind::EXPR_FILTER => print_expr_filter(node, ctx),
        SyntaxKind::EXPR_GET_ATTR => print_expr_get(node, ctx),
        SyntaxKind::EXPR_GET_ITEM => print_expr_get(node, ctx),
        SyntaxKind::EXPR_IDENT => print_expr_ident(node),
        SyntaxKind::EXPR_IF => print_expr_if(node, ctx),
        SyntaxKind::EXPR_LIST => print_expr_list(node, ctx),
        SyntaxKind::EXPR_LITERAL => print_expr_literal(node, ctx),
        SyntaxKind::EXPR_PAREN => print_expr_paren(node, ctx),
        SyntaxKind::EXPR_TEST => print_expr_test(node, ctx),
        SyntaxKind::EXPR_TUPLE => print_expr_tuple(node, ctx),
        SyntaxKind::EXPR_UNARY => print_expr_unary(node, ctx),
        SyntaxKind::PARAM => print_param(node, ctx),
        SyntaxKind::STMT_CALL => todo!(),
        SyntaxKind::STMT_FILTER => print_stmt_filter(node, ctx),
        SyntaxKind::STMT_FOR => print_stmt_for(node, ctx),
        SyntaxKind::STMT_MACRO => print_stmt_macro(node, ctx),
        SyntaxKind::STMT_SET => print_stmt_set(node, ctx),
        SyntaxKind::STMT_UNKNOWN => print_stmt_unknown(node, ctx),
        SyntaxKind::STMT_WITH => print_stmt_with(node, ctx),
        SyntaxKind::ROOT_EXPR => print_root(node, ctx),
        SyntaxKind::ROOT_STMT => print_root(node, ctx),
        _ => unreachable!("only syntax node is expected"),
    }
}

fn print_arg(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_without_whitespaces(node, ctx)
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

fn print_expr_call(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    node.first_child()
        .map(|node| print_node(&node, ctx))
        .unwrap_or_else(Doc::nil)
        .append(print_comma_separated_with_delimiter(
            node.children_with_tokens()
                .skip_while(|node_or_token| node_or_token.kind() != SyntaxKind::L_PAREN),
            ctx,
            ctx.options.args_trailing_comma,
            ctx.options.args_prefer_single_line,
            ctx.options.args_paren_spacing,
        ))
}

fn print_expr_concat(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_expr_with_operator(node, ctx).group()
}

fn print_expr_dict(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_comma_separated_with_delimiter(
        node.children_with_tokens(),
        ctx,
        ctx.options.expr_dict_trailing_comma,
        ctx.options.expr_dict_prefer_single_line,
        ctx.options.brace_spacing,
    )
}

fn print_expr_dict_item(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
            .map(|node_or_token| match node_or_token {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) if token.kind() == SyntaxKind::COLON => Doc::text(": "),
                NodeOrToken::Token(token) => Doc::text(token.text().to_string()),
            })
            .collect(),
    )
}

fn print_expr_filter(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_expr_with_operator(node, ctx).group()
}

fn print_expr_get(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_without_whitespaces(node, ctx)
}

fn print_expr_ident(node: &SyntaxNode) -> Doc<'static> {
    node.first_token()
        .map(|token| Doc::text(token.text().to_string()))
        .unwrap_or_else(Doc::nil)
}

fn print_expr_if(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
            .map(|node_or_token| match node_or_token {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) => {
                    if token.kind() == SyntaxKind::KEYWORD {
                        Doc::line_or_space()
                            .append(Doc::text(token.text().to_string()).append(Doc::space()))
                    } else {
                        Doc::text(token.text().to_string())
                    }
                }
            })
            .collect(),
    )
    .group()
}

fn print_expr_list(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_comma_separated_with_delimiter(
        node.children_with_tokens(),
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
    print_without_whitespaces(node, ctx)
}

fn print_expr_test(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_expr_with_operator(node, ctx).group()
}

fn print_expr_tuple(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_comma_separated_with_delimiter(
        node.children_with_tokens(),
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

fn print_expr_unary(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
            .map(|node_or_token| match node_or_token {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) => {
                    if token.kind() == SyntaxKind::OPERATOR {
                        Doc::text(token.text().to_string()).append(Doc::space())
                    } else {
                        Doc::text(token.text().to_string())
                    }
                }
            })
            .collect(),
    )
}

fn print_expr_with_operator(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
            .map(|node_or_token| match node_or_token {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) => {
                    if token.kind() == SyntaxKind::OPERATOR {
                        let (prefix, suffix) = get_operator_space(ctx);
                        prefix
                            .append(Doc::text(token.text().to_string()))
                            .append(suffix)
                    } else {
                        Doc::text(token.text().to_string())
                    }
                }
            })
            .collect(),
    )
}

fn print_param(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    print_without_whitespaces(node, ctx)
}

fn print_root(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    node.first_child()
        .map(|child| print_node(&child, ctx))
        .unwrap_or_else(Doc::nil)
}

fn print_stmt_filter(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::text("filter ")
        .append(
            support::token(node, SyntaxKind::IDENT)
                .map(|token| Doc::text(token.text().to_string()))
                .unwrap_or_else(Doc::nil),
        )
        .append(print_comma_separated_with_delimiter(
            node.children_with_tokens()
                .skip_while(|node_or_token| node_or_token.kind() != SyntaxKind::L_PAREN),
            ctx,
            ctx.options.params_trailing_comma,
            ctx.options.params_prefer_single_line,
            ctx.options.params_paren_spacing,
        ))
}

fn print_stmt_for(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
            .map(|node_or_token| match node_or_token {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) => match token.kind() {
                    SyntaxKind::KEYWORD => match token.text() {
                        "for" => Doc::text("for "),
                        "recursive" => Doc::text(" recursive"),
                        "if" => Doc::line_or_space().append(Doc::text("if ")),
                        text => Doc::text(format!(" {text} ")),
                    },
                    SyntaxKind::COMMA => Doc::text(", "),
                    _ => Doc::text(token.text().to_string()),
                },
            })
            .collect(),
    )
    .group()
}

fn print_stmt_macro(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::text("macro ")
        .append(
            support::token(node, SyntaxKind::IDENT)
                .map(|token| Doc::text(token.text().to_string()))
                .unwrap_or_else(Doc::nil),
        )
        .append(print_comma_separated_with_delimiter(
            node.children_with_tokens()
                .skip_while(|node_or_token| node_or_token.kind() != SyntaxKind::L_PAREN),
            ctx,
            ctx.options.params_trailing_comma,
            ctx.options.params_prefer_single_line,
            ctx.options.params_paren_spacing,
        ))
}

fn print_stmt_set(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
            .map(|node_or_token| match node_or_token {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) => match token.kind() {
                    SyntaxKind::KEYWORD => Doc::text(token.text().to_string()).append(Doc::space()),
                    SyntaxKind::EQ => Doc::space()
                        .append(Doc::text(token.text().to_string()))
                        .append(Doc::space()),
                    SyntaxKind::COMMA => Doc::text(token.text().to_string()).append(Doc::space()),
                    SyntaxKind::OPERATOR => {
                        let (prefix, suffix) = get_operator_space(ctx);
                        prefix
                            .append(Doc::text(token.text().to_string()))
                            .append(suffix)
                    }
                    _ => Doc::text(token.text().to_string()),
                },
            })
            .collect(),
    )
    .group()
}

fn print_stmt_unknown(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
            .map(|node_or_token| match node_or_token {
                NodeOrToken::Node(node) => {
                    let doc = print_node(&node, ctx);
                    if node
                        .siblings_with_tokens(Direction::Next)
                        .skip(1)
                        .find(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
                        .is_some_and(|node_or_token| matches!(node_or_token, NodeOrToken::Node(..)))
                    {
                        doc.append(Doc::space())
                    } else {
                        doc
                    }
                }
                NodeOrToken::Token(token) => match token.kind() {
                    SyntaxKind::KEYWORD | SyntaxKind::COMMA => {
                        Doc::text(token.text().to_string()).append(Doc::space())
                    }
                    _ => Doc::text(token.text().to_string()),
                },
            })
            .collect(),
    )
}

fn print_stmt_with(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
            .map(|node_or_token| match node_or_token {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) => match token.kind() {
                    SyntaxKind::KEYWORD => Doc::text(token.text().to_string()).append(Doc::space()),
                    SyntaxKind::EQ => Doc::space()
                        .append(Doc::text(token.text().to_string()))
                        .append(Doc::space()),
                    SyntaxKind::COMMA => Doc::text(token.text().to_string()).append(Doc::space()),
                    _ => Doc::text(token.text().to_string()),
                },
            })
            .collect(),
    )
}

fn print_without_whitespaces(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    Doc::list(
        node.children_with_tokens()
            .filter(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
            .map(|node_or_token| match node_or_token {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) => Doc::text(token.text().to_string()),
            })
            .collect(),
    )
}

fn print_comma_separated_with_delimiter(
    elements: impl Iterator<Item = NodeOrToken>,
    ctx: &Ctx,
    trailing_comma: Option<TrailingComma>,
    prefer_single_line: Option<bool>,
    delim_spacing: bool,
) -> Doc<'static> {
    Doc::list(
        elements
            .filter(|node_or_token| node_or_token.kind() != SyntaxKind::WHITESPACE)
            .map(|node_or_token| match node_or_token {
                NodeOrToken::Node(child) => {
                    let doc = print_node(&child, ctx);
                    let doc = if child.next_sibling().is_some() {
                        doc.append(Doc::text(",")).append(Doc::line_or_space())
                    } else {
                        let comma = match trailing_comma.unwrap_or(ctx.options.trailing_comma) {
                            TrailingComma::Never => Doc::nil(),
                            TrailingComma::Always => Doc::text(","),
                            TrailingComma::OnlyMultiLine => {
                                Doc::flat_or_break(Doc::nil(), Doc::text(","))
                            }
                        };
                        doc.append(comma)
                    };
                    doc.nest(ctx.indent_width)
                }
                NodeOrToken::Token(token) => {
                    if token.kind() == SyntaxKind::COMMA {
                        Doc::nil()
                    } else {
                        let doc = Doc::text(token.text().to_string());
                        match token.kind() {
                            SyntaxKind::L_PAREN | SyntaxKind::L_BRACKET | SyntaxKind::L_BRACE => {
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
                            }
                            SyntaxKind::R_PAREN | SyntaxKind::R_BRACKET | SyntaxKind::R_BRACE => {
                                let ws = if delim_spacing {
                                    Doc::line_or_space()
                                } else {
                                    Doc::line_or_nil()
                                };
                                ws.append(doc)
                            }
                            _ => doc,
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
