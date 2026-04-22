use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use wasmtime::{Config, Engine, Linker, Module, Store};

use crate::plugin_bindings::{DelimiterError, Span};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HostDelimiterError {
    Extra { ch: char, span: HostSpan },
    Missing { ch: char, span: HostSpan },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostSpan {
    pub start_byte: usize,
    pub end_byte: usize,
}

impl From<&crate::ast::DelimiterError> for HostDelimiterError {
    fn from(err: &crate::ast::DelimiterError) -> Self {
        match err {
            crate::ast::DelimiterError::Extra { span, delimiter } => HostDelimiterError::Extra {
                ch: *delimiter,
                span: HostSpan {
                    start_byte: span.start_byte,
                    end_byte: span.end_byte,
                },
            },
            crate::ast::DelimiterError::Missing { expected, insert_at, .. } => {
                HostDelimiterError::Missing {
                    ch: *expected,
                    span: HostSpan {
                        start_byte: insert_at.start_byte,
                        end_byte: insert_at.end_byte,
                    },
                }
            }
        }
    }
}

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

    pub fn repair(
        &self,
        errors: &[crate::ast::DelimiterError],
        source: &str,
    ) -> Result<String> {
        let host_errors: Vec<HostDelimiterError> = errors.iter().map(HostDelimiterError::from).collect();
        let input_json = serde_json::to_string(&(host_errors, source))?;

        let mut store = Store::new(&self.engine, ());
        let linker = Linker::new(&self.engine);
        let instance = linker.instantiate(&mut store, &self.module)?;

        let repair_func = instance
            .get_typed_func::<(String,), (String,)>(&mut store, "repair")
            .map_err(|_| anyhow!("Plugin does not export a 'repair' function"))?;

        let (output_json,) = repair_func.call(&mut store, (input_json,))?;
        Ok(output_json)
    }
}
