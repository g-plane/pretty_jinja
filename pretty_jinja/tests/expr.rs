use pretty_jinja::{
    config::{FormatOptions, LanguageOptions, LayoutOptions, OperatorLineBreak, TrailingComma},
    format_expr,
};
use similar_asserts::assert_eq;

#[test]
fn single_quote_string() {
    let input = "'ab'";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "'ab'");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn double_quote_string() {
    let input = "\"ab\"";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "\"ab\"");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn number() {
    let input = "123_456.789";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "123_456.789");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn boolean() {
    let input = "true";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "true");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn list() {
    let input = "[  1 ,  2 ]";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "[1, 2]");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn list_trailing_comma_never() {
    let input = "[\n1 ,  2 ]";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            expr_list_trailing_comma: Some(TrailingComma::Never),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "[\n  1,\n  2\n]");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn list_trailing_comma_always() {
    let input = "[1 ,  2 ]";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            expr_list_trailing_comma: Some(TrailingComma::Always),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "[1, 2,]");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn list_trailing_comma_only_multi_line() {
    let input = "[aaaaa,bbbbb]";
    let options = FormatOptions {
        layout: LayoutOptions {
            print_width: 10,
            ..Default::default()
        },
        language: LanguageOptions {
            expr_list_trailing_comma: Some(TrailingComma::OnlyMultiLine),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "[\n  aaaaa,\n  bbbbb,\n]");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn list_prefer_single_line() {
    let input = "[\n1,\n2]";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            expr_list_prefer_single_line: Some(true),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "[1, 2]");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn list_spacing() {
    let input = "[1,2]";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            bracket_spacing: true,
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "[ 1, 2 ]");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn tuple() {
    let input = "(  1 ,  2 )";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "(1, 2)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn tuple_trailing_comma_never() {
    let input = "(\n1 ,  2 )";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            expr_tuple_trailing_comma: Some(TrailingComma::Never),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "(\n  1,\n  2\n)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn tuple_trailing_comma_always() {
    let input = "(1 ,  2 )";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            expr_tuple_trailing_comma: Some(TrailingComma::Always),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "(1, 2,)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn tuple_trailing_comma_only_multi_line() {
    let input = "(aaaaa,bbbbb)";
    let options = FormatOptions {
        layout: LayoutOptions {
            print_width: 10,
            ..Default::default()
        },
        language: LanguageOptions {
            expr_tuple_trailing_comma: Some(TrailingComma::OnlyMultiLine),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "(\n  aaaaa,\n  bbbbb,\n)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn tuple_trailing_comma_never_for_single_component() {
    let input = "(1 ,   )";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            expr_tuple_trailing_comma: Some(TrailingComma::Never),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "(1,)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn tuple_prefer_single_line() {
    let input = "(\n1,\n2)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            expr_tuple_prefer_single_line: Some(true),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "(1, 2)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn tuple_spacing() {
    let input = "(1 ,  2)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            tuple_paren_spacing: true,
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "( 1, 2 )");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn dict() {
    let input = "{ 'dict' : 'of' , 'key':'and','value' : 'pairs'}";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "{'dict': 'of', 'key': 'and', 'value': 'pairs'}");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn dict_trailing_comma_never() {
    let input = "{\n'dict' : 'of' , 'key':'and','value' : 'pairs'}";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            expr_dict_trailing_comma: Some(TrailingComma::Never),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(
        output,
        "{\n  'dict': 'of',\n  'key': 'and',\n  'value': 'pairs'\n}"
    );
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn dict_trailing_comma_always() {
    let input = "{ 'dict' : 'of' , 'key':'and','value' : 'pairs'}";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            expr_dict_trailing_comma: Some(TrailingComma::Always),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "{'dict': 'of', 'key': 'and', 'value': 'pairs',}");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn dict_trailing_comma_only_multi_line() {
    let input = "{'dict' : 'of' , 'key':'and','value' : 'pairs'}";
    let options = FormatOptions {
        layout: LayoutOptions {
            print_width: 10,
            ..Default::default()
        },
        language: LanguageOptions {
            expr_dict_trailing_comma: Some(TrailingComma::OnlyMultiLine),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(
        output,
        "{\n  'dict': 'of',\n  'key': 'and',\n  'value': 'pairs',\n}"
    );
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn dict_prefer_single_line() {
    let input = "{\n'dict' : 'of' ,\n'key':'and',\n'value' : 'pairs'}";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            expr_dict_prefer_single_line: Some(true),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "{'dict': 'of', 'key': 'and', 'value': 'pairs'}");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn dict_spacing() {
    let input = "{'dict' : 'of' , 'key':'and','value' : 'pairs'}";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            brace_spacing: true,
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "{ 'dict': 'of', 'key': 'and', 'value': 'pairs' }");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn binary_expr() {
    let input = "1+2*3+4/5**6-7%8==true>false  and  not  false  or(x  in  y)";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(
        output,
        "1 + 2 * 3 + 4 / 5 ** 6 - 7 % 8 == true > false and not false or (x in y)"
    );
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn operator_linebreak_before() {
    let input = "aaaa + bbbb + cccc";
    let options = FormatOptions {
        layout: LayoutOptions {
            print_width: 10,
            ..Default::default()
        },
        language: LanguageOptions {
            operator_linebreak: OperatorLineBreak::Before,
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "aaaa\n  + bbbb\n  + cccc");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn operator_linebreak_after() {
    let input = "aaaa + bbbb + cccc";
    let options = FormatOptions {
        layout: LayoutOptions {
            print_width: 10,
            ..Default::default()
        },
        language: LanguageOptions {
            operator_linebreak: OperatorLineBreak::After,
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "aaaa +\n  bbbb +\n  cccc");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn test() {
    let input = "b  is  value";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "b is value");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn test_without_parens() {
    let input = "loop.index is divisibleby 3";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "loop.index is divisibleby(3)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn test_without_parens_too_long() {
    let input = "loop.index is divisibleby 3";
    let options = FormatOptions {
        layout: LayoutOptions {
            print_width: 10,
            ..Default::default()
        },
        language: Default::default(),
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "loop.index is\n  divisibleby(\n    3,\n  )");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn test_without_parens_trailing_comma_always() {
    let input = "loop.index is divisibleby 3";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_trailing_comma: Some(TrailingComma::Always),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "loop.index is divisibleby(3,)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn test_without_parens_paren_spacing() {
    let input = "loop.index is divisibleby 3";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_paren_spacing: true,
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "loop.index is divisibleby( 3 )");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn pipe() {
    let input = "a|b()  |   c()";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "a | b() | c()");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn concat() {
    let input = "a~b()  ~   c()";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "a ~ b() ~ c()");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn mixed_pipe_concat() {
    let input = "a~b()  |   c()  ~  d()|e()";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "a ~ b() | c() ~ d() | e()");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call() {
    let input = "post.render (1+2 , full = true)";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "post.render(1 + 2, full=true)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_trailing_comma_never() {
    let input = "post.render (\n1+2 , full = true)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_trailing_comma: Some(TrailingComma::Never),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "post.render(\n  1 + 2,\n  full=true\n)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_trailing_comma_always() {
    let input = "post.render (1+2 , full = true)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_trailing_comma: Some(TrailingComma::Always),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "post.render(1 + 2, full=true,)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_trailing_comma_only_multi_line() {
    let input = "post.render(1+2 , full = true)";
    let options = FormatOptions {
        layout: LayoutOptions {
            print_width: 10,
            ..Default::default()
        },
        language: LanguageOptions {
            args_trailing_comma: Some(TrailingComma::OnlyMultiLine),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "post.render(\n  1 + 2,\n  full=true,\n)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_prefer_single_line() {
    let input = "post.render(\n1+2 ,\nfull = true\n)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_prefer_single_line: Some(true),
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "post.render(1 + 2, full=true)");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_spacing() {
    let input = "post.render(1+2 , full =true)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_paren_spacing: true,
            ..Default::default()
        },
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "post.render( 1 + 2, full=true )");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn get_attr() {
    let input = "a() . b";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "a().b");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn get_item() {
    let input = "a() [ b*c ]";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "a()[b * c]");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn mixed_access() {
    let input = "a () [ b ] . c ( ) [ d ] () .e [f]";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(output, "a()[b].c()[d]().e[f]");
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn expr_if() {
    let input = "layout_template if layout_template is defined else 'default.html'";
    let options = Default::default();
    let output = format_expr(input, &options).unwrap();
    assert_eq!(
        output,
        "layout_template if layout_template is defined else 'default.html'"
    );
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn expr_if_too_long() {
    let input = "layout_template if layout_template is defined else 'default.html'";
    let options = FormatOptions {
        layout: LayoutOptions {
            print_width: 30,
            ..Default::default()
        },
        language: Default::default(),
    };
    let output = format_expr(input, &options).unwrap();
    assert_eq!(
        output,
        "layout_template\nif layout_template is defined\nelse 'default.html'"
    );
    assert_eq!(
        format_expr(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}
