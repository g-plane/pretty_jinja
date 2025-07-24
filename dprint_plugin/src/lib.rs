use crate::config::resolve_config;
use anyhow::Result;
use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration},
    plugins::{
        CheckConfigUpdatesMessage, ConfigChange, FormatResult, PluginInfo,
        PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest,
        SyncPluginHandler,
    },
};
use pretty_jinja::{config::FormatOptions, format_expr, format_stmt};

mod config;

pub struct PrettyJinjaPluginHandler;

impl SyncPluginHandler<FormatOptions> for PrettyJinjaPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version.clone(),
            config_key: "jinja".to_string(),
            help_url: "https://github.com/g-plane/pretty_jinja".to_string(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/g-plane/pretty_jinja/v{version}/schema.json"
            ),
            update_url: Some("https://plugins.dprint.dev/g-plane/pretty_jinja/latest.json".into()),
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../../LICENSE").into()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<FormatOptions> {
        resolve_config(config, global_config)
    }

    fn check_config_updates(&self, _: CheckConfigUpdatesMessage) -> Result<Vec<ConfigChange>> {
        Ok(Vec::new())
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<FormatOptions>,
        _: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        match request.file_path.extension().and_then(|s| s.to_str()) {
            Some("markup-fmt-jinja-expr") => {
                format_expr(std::str::from_utf8(&request.file_bytes)?, request.config)
                    .map(|output| Some(output.into_bytes()))
                    .map_err(|error| anyhow::anyhow!(error))
            }
            Some("markup-fmt-jinja-stmt") => {
                format_stmt(std::str::from_utf8(&request.file_bytes)?, request.config)
                    .map(|output| Some(output.into_bytes()))
                    .map_err(|error| anyhow::anyhow!(error))
            }
            _ => Ok(None),
        }
    }
}

#[cfg(target_arch = "wasm32")]
dprint_core::generate_plugin_code!(
    PrettyJinjaPluginHandler,
    PrettyJinjaPluginHandler,
    FormatOptions
);
