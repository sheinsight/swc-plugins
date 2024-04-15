use auto_css_modules::auto_css_modules::{AutoCssModulesConfig, CssModulesTransform};
use swc_core::ecma::{
    ast::Program,
    visit::{as_folder, FoldWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub auto_css_modules_config: AutoCssModulesConfig,
}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config_str = &metadata
        .get_transform_plugin_config()
        .expect("failed to get config");
    let config = serde_json::from_str::<Config>(config_str).expect("invalid packages");

    if config.auto_css_modules_config.enable {
        program.fold_with(&mut as_folder(CssModulesTransform {
            config: config.auto_css_modules_config,
        }))
    } else {
        program
    }
}
