use serde::Deserialize;
use swc_core::ecma::{ast::ImportDecl, visit::VisitMut};

#[derive(Clone, Debug, Deserialize)]
pub struct LockModulesConfig {
    #[serde(default = "get_default_enable")]
    pub enable: bool,

    pub source: String,

    pub target: String,
}

fn get_default_enable() -> bool {
    false
}

pub struct LockModules {
    pub config: LockModulesConfig,
}

impl VisitMut for LockModules {
    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        let src_value = import_decl.src.value.as_str();
        if import_decl.specifiers.is_empty() {
            src_value.starts_with(self.config.source.as_str());
            let source = src_value.replace(&self.config.source, &self.config.target);
            import_decl.src = Box::new(source.into());
        }
    }
}
