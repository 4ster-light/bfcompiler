use bf_interpreter::{Interpreter, RunStatus};
use wasm_bindgen::prelude::*;

fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}

/// A resumable interpreter owned by the browser playground.
#[wasm_bindgen]
pub struct WebInterpreter {
    inner: Interpreter,
}

#[wasm_bindgen]
impl WebInterpreter {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str, input: &[u8]) -> Result<WebInterpreter, JsValue> {
        Interpreter::new(source, input.to_vec())
            .map(|inner| Self { inner })
            .map_err(js_error)
    }

    /// Execute a bounded batch. Returning keeps the browser event loop responsive.
    pub fn run(&mut self, max_steps: u32) -> Result<String, JsValue> {
        self.inner
            .run(u64::from(max_steps))
            .map(|status| match status {
                RunStatus::Running => "running",
                RunStatus::Finished => "finished",
            })
            .map(String::from)
            .map_err(js_error)
    }

    pub fn step(&mut self) -> Result<String, JsValue> {
        self.run(1)
    }

    #[wasm_bindgen(getter)]
    pub fn output(&self) -> Vec<u8> {
        self.inner.output().to_vec()
    }

    #[wasm_bindgen(getter)]
    pub fn pointer(&self) -> usize {
        self.inner.pointer()
    }

    #[wasm_bindgen(getter)]
    pub fn instruction_pointer(&self) -> usize {
        self.inner.instruction_pointer()
    }

    #[wasm_bindgen(getter)]
    pub fn steps(&self) -> u64 {
        self.inner.steps()
    }

    #[wasm_bindgen(getter)]
    pub fn current_value(&self) -> u8 {
        self.inner
            .tape()
            .get(self.inner.pointer())
            .copied()
            .unwrap_or(0)
    }

    pub fn memory(&self, start: usize, length: usize) -> Vec<u8> {
        let tape = self.inner.tape();
        tape.iter().skip(start).take(length).copied().collect()
    }
}

/// Build metadata displayed by the web UI without a second source of truth.
#[wasm_bindgen]
pub fn tape_size() -> usize {
    bf_interpreter::TAPE_SIZE
}
