use crate::syntax::{JinjaLanguage, SyntaxKind};
use rowan::{GreenNode, GreenToken, NodeOrToken};
use winnow::{
    Parser,
    ascii::{line_ending, multispace1, take_escaped},
    combinator::{alt, eof, opt, peek, repeat, terminated},
    error::{ContextError, ParseError},
    stream::AsChar,
    token::{any, none_of, one_of, take_while},
};

type SyntaxNode = rowan::SyntaxNode<JinjaLanguage>;
type GreenElement = NodeOrToken<GreenNode, GreenToken>;
type GreenResult = winnow::Result<GreenElement>;
type Input<'s> = &'s str;

fn tok(kind: SyntaxKind, text: &str) -> GreenElement {
    NodeOrToken::Token(GreenToken::new(kind.into(), text))
}
fn node<I>(kind: SyntaxKind, children: I) -> GreenElement
where
    I: IntoIterator<Item = GreenElement>,
    I::IntoIter: ExactSizeIterator,
{
    NodeOrToken::Node(GreenNode::new(kind.into(), children))
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || !c.is_ascii()
}

fn whitespace(input: &mut Input) -> GreenResult {
    multispace1
        .parse_next(input)
        .map(|text| tok(SyntaxKind::WHITESPACE, text))
}

fn bool(input: &mut Input) -> GreenResult {
    terminated(alt(("true", "false")), peek(none_of(is_ident_char)))
        .parse_next(input)
        .map(|text| tok(SyntaxKind::BOOL, text))
}

fn ident(input: &mut Input) -> GreenResult {
    (
        one_of(|c: char| c.is_ascii_alphabetic() || c == '_' || !c.is_ascii()),
        take_while(0.., is_ident_char),
    )
        .take()
        .parse_next(input)
        .map(|text| tok(SyntaxKind::IDENT, text))
}

fn number(input: &mut Input) -> GreenResult {
    (
        opt(one_of(['+', '-'])),
        (
            (unsigned_dec, opt(('.', opt(unsigned_dec)))),
            opt((one_of(['e', 'E']), opt(one_of(['+', '-'])), unsigned_dec)),
        )
            .void(),
        peek(none_of(is_ident_char)),
    )
        .take()
        .parse_next(input)
        .map(|text| tok(SyntaxKind::NUMBER, text))
}
fn unsigned_dec<'s>(input: &mut Input<'s>) -> winnow::Result<&'s str> {
    (
        one_of(AsChar::is_dec_digit),
        take_while(0.., |c: char| c.is_ascii_digit() || c == '_'),
    )
        .take()
        .parse_next(input)
}

fn string(input: &mut Input) -> GreenResult {
    alt((
        (
            '"',
            take_escaped(none_of(['"', '\\', '\n', '\r']), '\\', any),
            alt(("\"", peek(line_ending), eof)),
        ),
        (
            '\'',
            take_escaped(none_of(['\'', '\\', '\n', '\r']), '\\', any),
            alt(("'", peek(line_ending), eof)),
        ),
    ))
    .take()
    .parse_next(input)
    .map(|text| tok(SyntaxKind::STRING, text))
}

fn expr(input: &mut Input) -> GreenResult {
    expr_bin.parse_next(input)
}

fn expr_access(input: &mut Input) -> GreenResult {
    (
        expr_term,
        repeat::<_, _, Vec<_>, _, _>(
            0..,
            alt((
                (opt(whitespace), '.', opt(whitespace), expr_term).map(
                    |(ws_leading, _, ws, expr)| {
                        let mut children = Vec::with_capacity(2);
                        if let Some(ws) = ws_leading {
                            children.push(ws);
                        }
                        children.push(tok(SyntaxKind::DOT, "."));
                        if let Some(ws) = ws {
                            children.push(ws);
                        }
                        children.push(expr);
                        (SyntaxKind::EXPR_GET_ATTR, children)
                    },
                ),
                (
                    opt(whitespace),
                    '[',
                    opt(whitespace),
                    expr,
                    opt(whitespace),
                    ']',
                )
                    .map(|(ws_leading, _, ws_before, expr, ws_after, _)| {
                        let mut children = Vec::with_capacity(3);
                        if let Some(ws) = ws_leading {
                            children.push(ws);
                        }
                        children.push(tok(SyntaxKind::L_BRACKET, "["));
                        if let Some(ws) = ws_before {
                            children.push(ws);
                        }
                        children.push(expr);
                        if let Some(ws) = ws_after {
                            children.push(ws);
                        }
                        children.push(tok(SyntaxKind::R_BRACKET, "]"));
                        (SyntaxKind::EXPR_GET_ITEM, children)
                    }),
                args.map(|args| (SyntaxKind::EXPR_CALL, args)),
            )),
        ),
    )
        .parse_next(input)
        .map(|(base, accesses)| {
            accesses
                .into_iter()
                .fold(base, |base, (kind, mut elements)| {
                    let mut children = Vec::with_capacity(1 + elements.len());
                    children.push(base);
                    children.append(&mut elements);
                    node(kind, children)
                })
        })
}
fn args(input: &mut Input) -> winnow::Result<Vec<GreenElement>> {
    (
        opt(whitespace),
        '(',
        repeat::<_, _, Vec<_>, _, _>(
            0..,
            (
                opt(whitespace),
                expr,
                alt((
                    (opt(whitespace), ','.map(|_| tok(SyntaxKind::COMMA, ","))).map(Some),
                    peek((opt(whitespace), ')')).value(None),
                )),
            ),
        ),
        opt(whitespace),
        ')',
    )
        .parse_next(input)
        .map(|(ws_leading, _, args, ws_after, _)| {
            let mut children = Vec::with_capacity(2 + args.len() * 3);
            if let Some(ws) = ws_leading {
                children.push(ws);
            }
            children.push(tok(SyntaxKind::L_PAREN, "("));
            args.into_iter().for_each(|(ws_before, expr, comma)| {
                if let Some(ws) = ws_before {
                    children.push(ws);
                }
                children.push(expr);
                if let Some((ws, comma)) = comma {
                    if let Some(ws) = ws {
                        children.push(ws);
                    }
                    children.push(comma);
                }
            });
            if let Some(ws) = ws_after {
                children.push(ws);
            }
            children.push(tok(SyntaxKind::R_PAREN, ")"));
            children
        })
}

fn expr_bin(input: &mut Input) -> GreenResult {
    expr_bin_or.parse_next(input)
}
fn expr_bin_common<'s, P1, P2>(
    operand: P1,
    operator: P2,
) -> impl Parser<Input<'s>, GreenElement, ContextError>
where
    P1: Parser<Input<'s>, GreenElement, ContextError> + Clone,
    P2: Parser<Input<'s>, &'s str, ContextError>,
{
    (
        operand.clone(),
        repeat::<_, _, Vec<_>, _, _>(
            0..,
            (
                opt(whitespace),
                operator.map(|text| tok(SyntaxKind::OPERATOR, text)),
                opt(whitespace),
                operand,
            ),
        ),
    )
        .map(|(left, rights)| {
            rights.into_iter().fold(left, |left, right| {
                let (ws_before, operator, ws_after, right) = right;
                let mut children = Vec::with_capacity(5);
                children.push(left);
                if let Some(ws) = ws_before {
                    children.push(ws);
                }
                children.push(operator);
                if let Some(ws) = ws_after {
                    children.push(ws);
                }
                children.push(right);
                node(SyntaxKind::EXPR_BIN, children)
            })
        })
}
fn expr_bin_pow(input: &mut Input) -> GreenResult {
    expr_bin_common(expr_unary, "**").parse_next(input)
}
fn expr_bin_mul(input: &mut Input) -> GreenResult {
    expr_bin_common(expr_bin_pow, alt(("*", "/", "//", "%"))).parse_next(input)
}
fn expr_bin_add(input: &mut Input) -> GreenResult {
    expr_bin_common(expr_bin_mul, alt(("+", "-"))).parse_next(input)
}
fn expr_bin_cmp(input: &mut Input) -> GreenResult {
    expr_bin_common(
        expr_bin_add,
        alt((
            "==",
            "!=",
            ('>', opt('=')).take(),
            ('<', opt('=')).take(),
            "in",
        )),
    )
    .parse_next(input)
}
fn expr_bin_and(input: &mut Input) -> GreenResult {
    expr_bin_common(
        expr_bin_cmp,
        terminated("and", peek(none_of(is_ident_char))),
    )
    .parse_next(input)
}
fn expr_bin_or(input: &mut Input) -> GreenResult {
    expr_bin_common(expr_bin_and, terminated("or", peek(none_of(is_ident_char)))).parse_next(input)
}

fn expr_ident(input: &mut Input) -> GreenResult {
    ident
        .parse_next(input)
        .map(|token| node(SyntaxKind::EXPR_IDENT, [token]))
}

fn expr_literal(input: &mut Input) -> GreenResult {
    alt((bool, number, string))
        .parse_next(input)
        .map(|token| node(SyntaxKind::EXPR_LITERAL, [token]))
}

fn expr_pipe(input: &mut Input) -> GreenResult {
    (
        expr_access,
        repeat::<_, _, Vec<_>, _, _>(
            0..,
            (
                opt(whitespace),
                '|'.map(|_| tok(SyntaxKind::OPERATOR, "|")),
                opt(whitespace),
                (ident, opt((opt(whitespace), args))).map(|(ident, args)| {
                    let mut children = Vec::with_capacity(2);
                    children.push(ident);
                    if let Some((ws, mut args)) = args {
                        if let Some(ws) = ws {
                            children.push(ws);
                        }
                        children.append(&mut args);
                        node(SyntaxKind::EXPR_CALL, children)
                    } else {
                        node(SyntaxKind::EXPR_IDENT, children)
                    }
                }),
            ),
        ),
    )
        .parse_next(input)
        .map(|(base, filters)| {
            if filters.is_empty() {
                base
            } else {
                let mut children = Vec::with_capacity(1 + filters.len() * 2);
                children.push(base);
                filters
                    .into_iter()
                    .for_each(|(ws_before, operator, ws_after, filter)| {
                        if let Some(ws) = ws_before {
                            children.push(ws);
                        }
                        children.push(operator);
                        if let Some(ws) = ws_after {
                            children.push(ws);
                        }
                        children.push(filter);
                    });
                node(SyntaxKind::EXPR_PIPE, children)
            }
        })
}

fn expr_paren(input: &mut Input) -> GreenResult {
    ("(", opt(whitespace), expr, opt(whitespace), ")")
        .parse_next(input)
        .map(|(l_paren, ws_before, expr, ws_after, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(tok(SyntaxKind::L_PAREN, l_paren));
            if let Some(ws) = ws_before {
                children.push(ws);
            }
            children.push(expr);
            if let Some(ws) = ws_after {
                children.push(ws);
            }
            children.push(tok(SyntaxKind::R_PAREN, r_paren));
            node(SyntaxKind::EXPR_PAREN, children)
        })
}

fn expr_term(input: &mut Input) -> GreenResult {
    alt((expr_literal, expr_ident, expr_paren)).parse_next(input)
}

fn expr_unary(input: &mut Input) -> GreenResult {
    alt((expr_unary_not, expr_pipe)).parse_next(input)
}
fn expr_unary_not(input: &mut Input) -> GreenResult {
    (
        "not",
        peek(none_of(is_ident_char)),
        opt(whitespace),
        expr_pipe,
    )
        .parse_next(input)
        .map(|(operator, _, ws, expr)| {
            let mut children = Vec::with_capacity(3);
            children.push(tok(SyntaxKind::OPERATOR, operator));
            if let Some(ws) = ws {
                children.push(ws);
            }
            children.push(expr);
            node(SyntaxKind::EXPR_UNARY, children)
        })
}

fn root(input: &mut Input) -> winnow::Result<GreenNode> {
    (opt(whitespace), expr, opt(whitespace))
        .parse_next(input)
        .map(|(ws_before, expr, ws_after)| {
            let mut children = Vec::with_capacity(3);
            if let Some(ws) = ws_before {
                children.push(ws);
            }
            children.push(expr);
            if let Some(ws) = ws_after {
                children.push(ws);
            }
            GreenNode::new(SyntaxKind::ROOT.into(), children)
        })
}

#[doc(hidden)]
pub fn parse(code: &str) -> Result<SyntaxNode, ParseError<Input<'_>, ContextError>> {
    let code = code.trim_start_matches('\u{feff}');
    root.parse(code).map(SyntaxNode::new_root)
}
