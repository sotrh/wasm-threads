mod thread;

use std::time::Duration;

use wasm_bindgen::prelude::*;

use crate::thread::spawn;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    wasm_tracing::set_as_global_default();

    let _handle1 = spawn(|| {
        let mut count = 0;
        loop {
            tracing::info!("From a thread! {count}");
            std::thread::sleep(Duration::from_millis(500));
            count += 1;
        }
    });

    let _handle2 = spawn(|| {
        let mut count = 0;
        loop {
            tracing::info!("From a different thread! {count}");
            std::thread::sleep(Duration::from_millis(1000));
            count += 1;
        }
    });
}
