
import init, * as wasm_bindgen from './pkg/wasm_threads.js';

console.log("Starting");
async function init_wasm_in_worker() {
    self.onmessage = async event => {
        console.log("received wasm");
        
        try {
            await init(event.data);
        } catch (err) {
            // propagate error to main thread and quit out
            setTimeout(() => { throw err; });
            throw err;
        }

        self.onmessage = async event => {
            // prevent attempting to execute multiple functions
            // on this worker
            self.onmessage = () => {};

            console.log("starting wasm");
            wasm_bindgen.worker_entry_point(event.data);
        }
    }
}

init_wasm_in_worker();