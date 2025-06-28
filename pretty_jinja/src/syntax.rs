#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(hidden)]
pub enum JinjaLanguage {}
impl rowan::Language for JinjaLanguage {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum SyntaxKind {
    // SyntaxToken
    WHITESPACE = 0,
    BOOL,
    IDENT,
    NUMBER,
    STRING,
    L_PAREN,
    R_PAREN,
    L_BRACKET,
    R_BRACKET,
    COMMA,
    DOT,
    KEYWORD,
    OPERATOR,

    // SyntaxNode
    EXPR_BIN,
    EXPR_CALL,
    EXPR_CONCAT,
    EXPR_FILTER,
    EXPR_GET_ATTR,
    EXPR_GET_ITEM,
    EXPR_IDENT,
    EXPR_LITERAL,
    EXPR_PAREN,
    EXPR_UNARY,
    ROOT,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}
