use anyhow::Result;
use std::path::Path;
use wasmtime::{Config, Engine, Module};

#[allow(dead_code)]
pub struct PluginHost {
    engine: Engine,
    module: Module,
}

impl PluginHost {
    pub fn load(path: &Path) -> Result<Self> {
        let mut config = Config::default();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;
        let module = Module::from_file(&engine, path)?;
        Ok(Self { engine, module })
    }

    // Stub implementation – actual plugin execution is postponed.
    pub fn repair(
        &self,
        _errors: &[crate::ast::DelimiterError],
        source: &str,
    ) -> Result<String> {
        // TODO: Activate plugin call once wasmtime component API is stable.
        Ok(source.to_string())
    }
}
