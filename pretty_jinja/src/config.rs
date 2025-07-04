#[cfg(feature = "config_serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
pub struct FormatOptions {
    #[cfg_attr(feature = "config_serde", serde(flatten))]
    pub layout: LayoutOptions,
    #[cfg_attr(feature = "config_serde", serde(flatten))]
    pub language: LanguageOptions,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// Configuration related to layout, such as indentation or print width.
pub struct LayoutOptions {
    #[cfg_attr(feature = "config_serde", serde(alias = "printWidth"))]
    pub print_width: usize,

    #[cfg_attr(feature = "config_serde", serde(alias = "useTabs"))]
    pub use_tabs: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "indentWidth"))]
    pub indent_width: usize,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "lineBreak", alias = "linebreak")
    )]
    pub line_break: LineBreak,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            print_width: 80,
            use_tabs: false,
            indent_width: 2,
            line_break: LineBreak::Lf,
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum LineBreak {
    #[default]
    Lf,
    Crlf,
}

impl From<LayoutOptions> for tiny_pretty::PrintOptions {
    fn from(options: LayoutOptions) -> Self {
        tiny_pretty::PrintOptions {
            indent_kind: if options.use_tabs {
                tiny_pretty::IndentKind::Tab
            } else {
                tiny_pretty::IndentKind::Space
            },
            line_break: match options.line_break {
                LineBreak::Lf => tiny_pretty::LineBreak::Lf,
                LineBreak::Crlf => tiny_pretty::LineBreak::Crlf,
            },
            width: options.print_width,
            tab_size: options.indent_width,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// Configuration related to syntax.
pub struct LanguageOptions {
    #[cfg_attr(feature = "config_serde", serde(alias = "operatorLineBreak"))]
    pub operator_linebreak: OperatorLineBreak,

    #[cfg_attr(feature = "config_serde", serde(alias = "trailingComma"))]
    pub trailing_comma: TrailingComma,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "expr_dict.trailing_comma", alias = "exprDict.trailingComma")
    )]
    pub expr_dict_trailing_comma: Option<TrailingComma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "expr_list.trailing_comma", alias = "exprList.trailingComma")
    )]
    pub expr_list_trailing_comma: Option<TrailingComma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "expr_tuple.trailing_comma",
            alias = "exprTuple.trailingComma"
        )
    )]
    pub expr_tuple_trailing_comma: Option<TrailingComma>,

    #[cfg_attr(feature = "config_serde", serde(alias = "preferSingleLine"))]
    pub prefer_single_line: bool,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "expr_dict.prefer_single_line",
            alias = "exprDict.preferSingleLine"
        )
    )]
    pub expr_dict_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "expr_list.prefer_single_line",
            alias = "exprList.preferSingleLine"
        )
    )]
    pub expr_list_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "expr_tuple.prefer_single_line",
            alias = "exprTuple.preferSingleLine"
        )
    )]
    pub expr_tuple_prefer_single_line: Option<bool>,

    #[cfg_attr(feature = "config_serde", serde(alias = "braceSpacing"))]
    pub brace_spacing: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "bracketSpacing"))]
    pub bracket_spacing: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "tupleParenSpacing"))]
    pub tuple_paren_spacing: bool,
}

impl Default for LanguageOptions {
    fn default() -> Self {
        Self {
            operator_linebreak: OperatorLineBreak::default(),
            trailing_comma: TrailingComma::default(),
            expr_dict_trailing_comma: None,
            expr_list_trailing_comma: None,
            expr_tuple_trailing_comma: None,
            prefer_single_line: false,
            expr_dict_prefer_single_line: None,
            expr_list_prefer_single_line: None,
            expr_tuple_prefer_single_line: None,
            brace_spacing: false,
            bracket_spacing: false,
            tuple_paren_spacing: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum OperatorLineBreak {
    Before,
    #[default]
    After,
}

#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum TrailingComma {
    Never,
    Always,
    #[default]
    OnlyMultiLine,
}
