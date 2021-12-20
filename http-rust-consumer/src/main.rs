
use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

wit_bindgen_wasmtime::import!("../httpexp.wit");
use httpexp::*;

fn main() {
    // Setup wasmtime runtime
    let engine = Engine::default();
    let module = Module::from_file(&engine, "../httpexp/target/wasm32-wasi/release/httpexp.wasm").unwrap();

    // Our instantiation is not trivial, as we use the `.wit` file,
    // so we need to use a Linker.
    let mut linker = Linker::new(&engine);
    // In the linker, we have to add support for Wasi, as it is used by the plugin.
    wasmtime_wasi::add_to_linker(&mut linker, |(wasi, _plugin_data)| wasi).unwrap();
    // Create a Wasi context.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args().unwrap()
        .build();

    let mut store = Store::new(&engine, (wasi, httpexp::HttpexpData {}));

    let (plugin, instance) =
        Httpexp::instantiate(&mut store, &module, &mut linker, |(_wasi, plugin_data)| {
            plugin_data
        }).unwrap();

    println!("{:?}", instance);

    let req =   httpexp::Request {
        method: httpexp::Method::Get,
        uri: "https://google.com",
        headers: &Vec::new(),
         params: &Vec::new(),
         body: Some(&[0]), 
      };
    let p = plugin.request(&mut store, req).unwrap();
    println!("{:?}", p);

}