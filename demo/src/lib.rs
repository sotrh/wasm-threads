mod thread;

use wasm_bindgen::prelude::*;

use crate::thread::spawn;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    wasm_tracing::set_as_global_default();

    spawn(|| {
        tracing::info!("From a thread!");
    });

    spawn(|| {
        tracing::info!("From a different thread!");
    });
}