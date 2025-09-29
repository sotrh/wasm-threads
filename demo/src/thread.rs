use wasm_bindgen::prelude::*;

pub fn spawn(f: impl FnOnce() -> () + Send + 'static) {
    spawn_web_impl(Work::from_f(f));
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

fn spawn_web_impl(work: Work) {
    // Get a url to create the worker
    let script_bytes = include_str!("worker.js").as_bytes();
    let script_array = js_sys::Array::from_iter([js_sys::Uint8Array::from(script_bytes)]);

    let blob_properties = web_sys::BlobPropertyBag::new();
    blob_properties.set_type("application/javascript");

    let blob =
        web_sys::Blob::new_with_u8_array_sequence_and_options(&script_array, &blob_properties)
            .unwrap();

    let script_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let worker = web_sys::Worker::new(&script_url).unwrap();

    // Send the current WASM module and memory to the worker
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::module());
    array.push(&wasm_bindgen::memory());
    worker.post_message(&array).unwrap();

    let work = Box::new(work);
    let ptr = Box::into_raw(work);
    worker.post_message(&JsValue::from(ptr as u32)).unwrap();
}