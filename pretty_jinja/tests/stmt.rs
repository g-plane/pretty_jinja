use pretty_jinja::{
    config::{FormatOptions, LanguageOptions, LayoutOptions, TrailingComma},
    format_stmt,
};
use similar_asserts::assert_eq;

#[test]
fn for_simple() {
    let input = "for  key,value  in  my_dict|dictsort";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "for key, value in my_dict | dictsort");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn for_if() {
    let input = "for  user   in users if   not user . hidden";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "for user in users if not user.hidden");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn for_recursive() {
    let input = "for   item   in   sitemap  recursive";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "for item in sitemap recursive");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn if_simple() {
    let input = "if   loop . previtem  is   defined and value>loop.previtem";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "if loop.previtem is defined and value > loop.previtem"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn elif() {
    let input = "elif   loop . previtem  is   defined and value>loop.previtem";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "elif loop.previtem is defined and value > loop.previtem"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn macro_simple() {
    let input = "macro  input( name ,  value = '' , type= 'text'  , size =20, )";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "macro input(name, value='', type='text', size=20)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn macro_trailing_comma_never() {
    let input = "macro  input(\nname ,  value = '' , type= 'text'  , size =20, )";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            params_trailing_comma: Some(TrailingComma::Never),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "macro input(\n  name,\n  value='',\n  type='text',\n  size=20\n)"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn macro_trailing_comma_always() {
    let input = "macro  input(name ,  value = '' , type= 'text'  , size =20, )";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            params_trailing_comma: Some(TrailingComma::Always),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "macro input(name, value='', type='text', size=20,)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn macro_trailing_comma_only_multi_line() {
    let input = "macro  input(name ,  value = '' , type= 'text'  , size =20, )";
    let options = FormatOptions {
        layout: LayoutOptions {
            print_width: 10,
            ..Default::default()
        },
        language: LanguageOptions {
            params_trailing_comma: Some(TrailingComma::OnlyMultiLine),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "macro input(\n  name,\n  value='',\n  type='text',\n  size=20,\n)"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn macro_prefer_single_line() {
    let input = "macro  input(\nname ,\n  value = '' , \ntype= 'text'  , size =20, )";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            params_prefer_single_line: Some(true),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "macro input(name, value='', type='text', size=20)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn macro_spacing() {
    let input = "macro  input(name , value = '' , type= 'text'  , size =20, )";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            params_paren_spacing: true,
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "macro input( name, value='', type='text', size=20 )"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call() {
    let input = "call  post (1+2 , full = true)";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "call post(1 + 2, full=true)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_trailing_comma_never() {
    let input = "call    post (\n1+2 , full = true)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_trailing_comma: Some(TrailingComma::Never),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "call post(\n  1 + 2,\n  full=true\n)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_trailing_comma_always() {
    let input = "call    post (1+2 , full = true)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_trailing_comma: Some(TrailingComma::Always),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "call post(1 + 2, full=true,)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_trailing_comma_only_multi_line() {
    let input = "call    post(1+2 , full = true)";
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
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "call post(\n  1 + 2,\n  full=true,\n)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_prefer_single_line() {
    let input = "call    post(\n1+2 ,\nfull = true\n)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_prefer_single_line: Some(true),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "call post(1 + 2, full=true)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_spacing() {
    let input = "call    post(1+2 , full =true)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_paren_spacing: true,
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "call post( 1 + 2, full=true )");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_params_trailing_comma_never() {
    let input = "call(\nname ,  value  , type  , size , ) dump_users(list_of_user)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            params_trailing_comma: Some(TrailingComma::Never),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "call(\n  name,\n  value,\n  type,\n  size\n) dump_users(list_of_user)"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_params_trailing_comma_always() {
    let input = "call(name ,  value  , type  , size  ) dump_users(list_of_user)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            params_trailing_comma: Some(TrailingComma::Always),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "call(name, value, type, size,) dump_users(list_of_user)"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_params_trailing_comma_only_multi_line() {
    let input = "call(name ,  value  , type  , size  ) dump_users(list_of_user)";
    let options = FormatOptions {
        layout: LayoutOptions {
            print_width: 30,
            ..Default::default()
        },
        language: LanguageOptions {
            params_trailing_comma: Some(TrailingComma::OnlyMultiLine),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "call(\n  name,\n  value,\n  type,\n  size,\n) dump_users(list_of_user)"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_params_prefer_single_line() {
    let input = "call(\nname ,\n  value  , \ntype  , size , ) dump_users(list_of_user)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            params_prefer_single_line: Some(true),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "call(name, value, type, size) dump_users(list_of_user)"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn call_params_spacing() {
    let input = "call(name , value  , type  , size) dump_users(list_of_user)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            params_paren_spacing: true,
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "call( name, value, type, size ) dump_users(list_of_user)"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn filter() {
    let input = " filter  upper ";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "filter upper");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn filter_trailing_comma_never() {
    let input = "filter    post (\n1+2 , full = true)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_trailing_comma: Some(TrailingComma::Never),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "filter post(\n  1 + 2,\n  full=true\n)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn filter_trailing_comma_always() {
    let input = "filter    post (1+2 , full = true)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_trailing_comma: Some(TrailingComma::Always),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "filter post(1 + 2, full=true,)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn filter_trailing_comma_only_multi_line() {
    let input = "filter    post(1+2 , full = true)";
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
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "filter post(\n  1 + 2,\n  full=true,\n)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn filter_prefer_single_line() {
    let input = "filter    post(\n1+2 ,\nfull = true\n)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_prefer_single_line: Some(true),
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "filter post(1 + 2, full=true)");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn filter_spacing() {
    let input = "filter    post(1+2 , full =true)";
    let options = FormatOptions {
        layout: Default::default(),
        language: LanguageOptions {
            args_paren_spacing: true,
            ..Default::default()
        },
    };
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "filter post( 1 + 2, full=true )");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn set() {
    let input = "set  key,value=[('index.html','Index'),('about.html','About')]";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "set key, value = [('index.html', 'Index'), ('about.html', 'About')]"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn set_block() {
    let input = "set  key,value";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "set key, value");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn set_with_filters() {
    let input = "set  key,value|wordwrap|upper";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "set key, value | wordwrap | upper");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn include() {
    let input = "include   \"sidebar.html\"   ignore     missing    without    context";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "include \"sidebar.html\" ignore missing without context"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn import_from() {
    let input = "from   'forms.html'   import  input   as   input_field ,textarea";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(
        output,
        "from 'forms.html' import input as input_field, textarea"
    );
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}

#[test]
fn import_with_context() {
    let input = "from   'forms.html'   import  input   with  context";
    let options = Default::default();
    let output = format_stmt(input, &options).unwrap();
    assert_eq!(output, "from 'forms.html' import input with context");
    assert_eq!(
        format_stmt(&output, &options).unwrap(),
        output,
        "format is unstable"
    );
}
