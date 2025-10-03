mod thread;

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::thread::spawn;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    wasm_tracing::set_as_global_default();

    let _handle1 = spawn(|| {
        tracing::info!("From a thread!");
    });

    let _handle2 = spawn(|| {
        tracing::info!("From a different thread!");
    });
}


async fn sleep(millis: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let closure = Rc::new(RefCell::new(None));
        let closure_clone = closure.clone();

        *closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            resolve.call0(&JsValue::NULL).unwrap();
            let _ = closure_clone.borrow_mut().take();
        }) as Box<dyn FnMut()>));

        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
                millis
            )
            .unwrap();
    });

    JsFuture::from(promise).await.unwrap();
}