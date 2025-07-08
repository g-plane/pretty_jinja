use dprint_core::configuration::{
    ConfigKeyMap, ConfigurationDiagnostic, GlobalConfiguration, NewLineKind,
    ResolveConfigurationResult, get_nullable_value, get_unknown_property_diagnostics, get_value,
};
use pretty_jinja::config::*;

pub(crate) fn resolve_config(
    mut config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<FormatOptions> {
    let mut diagnostics = Vec::new();
    let pretty_jinja_config = FormatOptions {
        layout: LayoutOptions {
            print_width: get_value(
                &mut config,
                "printWidth",
                global_config.line_width.unwrap_or(80),
                &mut diagnostics,
            ) as usize,
            use_tabs: get_value(
                &mut config,
                "useTabs",
                global_config.use_tabs.unwrap_or_default(),
                &mut diagnostics,
            ),
            indent_width: get_value(
                &mut config,
                "indentWidth",
                global_config.indent_width.unwrap_or(2),
                &mut diagnostics,
            ) as usize,
            line_break: match &*get_value(
                &mut config,
                "lineBreak",
                match global_config.new_line_kind {
                    Some(NewLineKind::LineFeed) => "lf",
                    Some(NewLineKind::CarriageReturnLineFeed) => "crlf",
                    _ => "lf",
                }
                .to_string(),
                &mut diagnostics,
            ) {
                "lf" => LineBreak::Lf,
                "crlf" => LineBreak::Crlf,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "lineBreak".into(),
                        message: "invalid value for config `lineBreak`".into(),
                    });
                    LineBreak::Lf
                }
            },
        },
        language: LanguageOptions {
            operator_linebreak: match &*get_value(
                &mut config,
                "operatorLinebreak",
                "after".to_string(),
                &mut diagnostics,
            ) {
                "before" => OperatorLineBreak::Before,
                "after" => OperatorLineBreak::After,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "operatorLinebreak".into(),
                        message: "invalid value for config `operatorLinebreak`".into(),
                    });
                    Default::default()
                }
            },
            trailing_comma: match &*get_value(
                &mut config,
                "trailingComma",
                "only-multi-line".to_string(),
                &mut diagnostics,
            ) {
                "never" => TrailingComma::Never,
                "always" => TrailingComma::Always,
                "only-multi-line" => TrailingComma::OnlyMultiLine,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "trailingComma".into(),
                        message: "invalid value for config `trailingComma`".into(),
                    });
                    Default::default()
                }
            },
            args_trailing_comma: get_nullable_value::<String>(
                &mut config,
                "args.trailingComma",
                &mut diagnostics,
            )
            .as_deref()
            .and_then(|option_value| match option_value {
                "never" => Some(TrailingComma::Never),
                "always" => Some(TrailingComma::Always),
                "only-multi-line" => Some(TrailingComma::OnlyMultiLine),
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "args.trailingComma".into(),
                        message: "invalid value for config `args.trailingComma`".into(),
                    });
                    Default::default()
                }
            }),
            expr_dict_trailing_comma: get_nullable_value::<String>(
                &mut config,
                "exprDict.trailingComma",
                &mut diagnostics,
            )
            .as_deref()
            .and_then(|option_value| match option_value {
                "never" => Some(TrailingComma::Never),
                "always" => Some(TrailingComma::Always),
                "only-multi-line" => Some(TrailingComma::OnlyMultiLine),
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "exprDict.trailingComma".into(),
                        message: "invalid value for config `exprDict.trailingComma`".into(),
                    });
                    Default::default()
                }
            }),
            expr_list_trailing_comma: get_nullable_value::<String>(
                &mut config,
                "exprList.trailingComma",
                &mut diagnostics,
            )
            .as_deref()
            .and_then(|option_value| match option_value {
                "never" => Some(TrailingComma::Never),
                "always" => Some(TrailingComma::Always),
                "only-multi-line" => Some(TrailingComma::OnlyMultiLine),
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "exprList.trailingComma".into(),
                        message: "invalid value for config `exprList.trailingComma`".into(),
                    });
                    Default::default()
                }
            }),
            expr_tuple_trailing_comma: get_nullable_value::<String>(
                &mut config,
                "exprTuple.trailingComma",
                &mut diagnostics,
            )
            .as_deref()
            .and_then(|option_value| match option_value {
                "never" => Some(TrailingComma::Never),
                "always" => Some(TrailingComma::Always),
                "only-multi-line" => Some(TrailingComma::OnlyMultiLine),
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "exprTuple.trailingComma".into(),
                        message: "invalid value for config `exprTuple.trailingComma`".into(),
                    });
                    Default::default()
                }
            }),
            params_trailing_comma: get_nullable_value::<String>(
                &mut config,
                "params.trailingComma",
                &mut diagnostics,
            )
            .as_deref()
            .and_then(|option_value| match option_value {
                "never" => Some(TrailingComma::Never),
                "always" => Some(TrailingComma::Always),
                "only-multi-line" => Some(TrailingComma::OnlyMultiLine),
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "params.trailingComma".into(),
                        message: "invalid value for config `params.trailingComma`".into(),
                    });
                    Default::default()
                }
            }),
            prefer_single_line: get_value(&mut config, "preferSingleLine", false, &mut diagnostics),
            args_prefer_single_line: get_nullable_value(
                &mut config,
                "args.preferSingleLine",
                &mut diagnostics,
            ),
            expr_dict_prefer_single_line: get_nullable_value(
                &mut config,
                "exprDict.preferSingleLine",
                &mut diagnostics,
            ),
            expr_list_prefer_single_line: get_nullable_value(
                &mut config,
                "exprList.preferSingleLine",
                &mut diagnostics,
            ),
            expr_tuple_prefer_single_line: get_nullable_value(
                &mut config,
                "exprTuple.preferSingleLine",
                &mut diagnostics,
            ),
            params_prefer_single_line: get_nullable_value(
                &mut config,
                "params.preferSingleLine",
                &mut diagnostics,
            ),
            brace_spacing: get_value(&mut config, "braceSpacing", false, &mut diagnostics),
            bracket_spacing: get_value(&mut config, "bracketSpacing", false, &mut diagnostics),
            args_paren_spacing: get_value(&mut config, "argsParenSpacing", false, &mut diagnostics),
            params_paren_spacing: get_value(
                &mut config,
                "paramsParenSpacing",
                false,
                &mut diagnostics,
            ),
            tuple_paren_spacing: get_value(
                &mut config,
                "tupleParenSpacing",
                false,
                &mut diagnostics,
            ),
        },
    };

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: pretty_jinja_config,
        diagnostics,
    }
}
