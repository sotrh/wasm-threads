importScripts('./pkg/wasm_threads.js');

async function init_wasm_in_worker() {
    
    self.onmessage = event => {
        console.log("received wasm");
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

            console.log("starting wasm");
            wasm_bindgen.worker_entry_point(event.data);
        }
    }
}

init_wasm_in_worker();