self.importScripts("pkg/wasm_threads.js");

async function init_wasm_in_worker() {
    self.onmessage = event => {
        let initialized = wasm_bindgen(...event.data).catch(err => {
            // propagate error to main thread and quit out
            setTimeout(() => { throw err; });
            throw err;
        });

        self.onmessage = async event => {
            // prevent attempting to execute multiple functions
            // on this worker
            self.onmessage = () => {};

            await initialized;

            wasm_bindgen.worker_entry_point(event.data);
        }
    }
}

init_wasm_in_worker();