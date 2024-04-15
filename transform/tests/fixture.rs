use auto_css_modules::{
    auto_css_modules::{AutoCssModulesConfig, CssModulesTransform},
    lock_module::{LockModules, LockModulesConfig},
};
use std::path::PathBuf;
use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{EsConfig, Syntax},
        transforms::{base::resolver, testing::test_fixture},
        visit::as_folder,
    },
};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixtures/auto_css_modules/**/input.js")]
fn fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    test_fixture(
        syntax(),
        &|_t| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                as_folder(CssModulesTransform {
                    config: AutoCssModulesConfig {
                        enable: true,
                        module_suffix: "?modules".into(),
                    }
                })
            )
        },
        &input,
        &dir.join("output.js"),
        Default::default(),
    );
}

#[fixture("tests/fixtures/lock/**/input.js")]
fn fixture_lock(input: PathBuf) {
    let dir = input.parent().unwrap();
    test_fixture(
        syntax(),
        &|_t| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                as_folder(LockModules {
                    config: LockModulesConfig {
                        enable: true,
                        // module_suffix: "?modules".into(),
                        source: "core-js/".into(),
                        target: "node_modules/core-js/".into()
                    }
                })
            )
        },
        &input,
        &dir.join("output.js"),
        Default::default(),
    );
}
