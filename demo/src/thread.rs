use wasm_bindgen::prelude::*;

pub struct Handle {
    #[allow(dead_code)]
    worker: web_sys::Worker,
}

pub fn spawn(f: impl FnOnce() -> () + Send + 'static) -> Handle {
    spawn_web_impl(Work::from_f(f))
}

/// Wrapper around a function pointer
struct Work {
    func: Box<dyn FnOnce() + Send>,
}

impl Work {
    fn from_f(f: impl FnOnce() + Send + 'static) -> Self {
        Self { func: Box::new(f) }
    }
}

#[wasm_bindgen]
pub fn worker_entry_point(ptr: u32) {
    let ptr = unsafe { Box::from_raw(ptr as *mut Work) };
    (ptr.func)();
}

fn spawn_web_impl(work: Work) -> Handle {
    let worker = web_sys::Worker::new("worker.js").unwrap();

    // Send the current WASM module and memory to the worker
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::module());
    array.push(&wasm_bindgen::memory());
    worker.post_message(&array).unwrap();

    let work = Box::new(work);
    let ptr = Box::into_raw(work);
    worker.post_message(&JsValue::from(ptr as u32)).unwrap();

    Handle { worker }
}
