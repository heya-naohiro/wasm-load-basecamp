use anyhow::Result;
use std::env;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

// use default wit dir
wasmtime::component::bindgen!();

#[derive(Default)]
struct State {}

impl HostWorldImports for State {
    fn print(&mut self, msg: String) {
        println!("[Host]WasmLog: {}", msg);
    }
}

fn main() -> Result<()> {
    let file = env::args().skip(1).next().unwrap_or_default();
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, file)?;
    let mut linker = Linker::new(&engine);

    HostWorld::add_to_linker(&mut linker, |s: &mut State| s)?;

    let mut store = Store::new(&engine, State::default());

    let (bindings, _) = HostWorld::instantiate(&mut store, &component, &linker)?;
    bindings.call_run(&mut store)?;
    Ok(())
}

fn create_config() -> Config {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config
}
