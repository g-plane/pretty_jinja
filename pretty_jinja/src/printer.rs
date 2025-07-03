use crate::{
    config::{FormatOptions, LanguageOptions},
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
        SyntaxKind::EXPR_CONCAT => todo!(),
        SyntaxKind::EXPR_DICT => todo!(),
        SyntaxKind::EXPR_DICT_ITEM => todo!(),
        SyntaxKind::EXPR_FILTER => todo!(),
        SyntaxKind::EXPR_GET_ATTR => print_expr_get(node, ctx),
        SyntaxKind::EXPR_GET_ITEM => print_expr_get(node, ctx),
        SyntaxKind::EXPR_IDENT => print_expr_ident(node),
        SyntaxKind::EXPR_IF => todo!(),
        SyntaxKind::EXPR_LIST => todo!(),
        SyntaxKind::EXPR_LITERAL => print_expr_literal(node, ctx),
        SyntaxKind::EXPR_PAREN => print_expr_paren(node, ctx),
        SyntaxKind::EXPR_TEST => todo!(),
        SyntaxKind::EXPR_TUPLE => todo!(),
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
    use crate::config::OperatorLineBreak;

    let doc = Doc::list(
        node.children_with_tokens()
            .filter(|element| element.kind() != SyntaxKind::WHITESPACE)
            .map(|element| match element {
                NodeOrToken::Node(node) => print_node(&node, ctx),
                NodeOrToken::Token(token) if token.kind() == SyntaxKind::OPERATOR => {
                    let (prefix, suffix) = match ctx.options.operator_linebreak {
                        OperatorLineBreak::Before => {
                            (Doc::line_or_space().nest(ctx.indent_width), Doc::space())
                        }
                        OperatorLineBreak::After => {
                            (Doc::space(), Doc::line_or_space().nest(ctx.indent_width))
                        }
                    };
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
    );
    if node
        .parent()
        .is_some_and(|node| node.kind() == SyntaxKind::EXPR_BIN)
    {
        doc
    } else {
        doc.group()
    }
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

fn print_root_expr(node: &SyntaxNode, ctx: &Ctx) -> Doc<'static> {
    node.first_child()
        .map(|child| print_node(&child, ctx))
        .unwrap_or_else(Doc::nil)
}
