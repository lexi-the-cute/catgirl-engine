// use wasm_bindgen::{prelude::wasm_bindgen, JsError};
// use std::io::Result;
use core::fmt::Error;
use core::result::Result;

#[derive(Debug)]
pub struct Builder {
    name: Option<String>,
    stack_size: Option<usize>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            name: None,
            stack_size: None,
        }
    }

    #[allow(unused_variables, unreachable_code)]
    pub fn spawn<F, T>(self, f: F) -> Result<JoinHandle<T>, Error> {
        // Result

        Ok(JoinHandle(todo!()))
    }

    pub fn name(mut self, name: String) -> Builder {
        self.name = Some(name);

        return self;
    }

    pub fn stack_size(mut self, size: usize) -> Builder {
        self.stack_size = Some(size);

        return self;
    }
}

pub struct JoinHandle<T>(T);

impl<T> JoinHandle<T> {
    pub fn is_finished(&self) -> bool {
        return false;
    }
}
