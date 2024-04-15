use std::ops::Not;

use serde::Deserialize;
use swc_core::ecma::{ast::ImportDecl, visit::VisitMut};

#[derive(Clone, Debug, Deserialize)]
pub struct AutoCssModulesConfig {
    #[serde(default = "get_default_enable")]
    pub enable: bool,
    #[serde(rename = "moduleSuffix", default = "get_default_module_suffix")]
    pub module_suffix: String,
}

fn get_default_enable() -> bool {
    false
}

fn get_default_module_suffix() -> String {
    "?modules".into()
}

const CSS_EXT_NAMES: [&str; 4] = [".css", ".less", ".sass", ".scss"];

pub struct CssModulesTransform {
    pub config: AutoCssModulesConfig,
}

impl VisitMut for CssModulesTransform {
    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        let src_value = import_decl.src.value.as_str();
        if import_decl.specifiers.is_empty().not() {
            let is_css = CSS_EXT_NAMES.iter().any(|ext| src_value.ends_with(ext));
            if is_css {
                let source = format!("{}{}", src_value, self.config.module_suffix);
                import_decl.src = Box::new(source.into());
            }
        }
    }
}
