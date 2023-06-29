// use wasm_bindgen::{prelude::wasm_bindgen, JsError};
// use std::io::Result;
// use core::fmt::Error;
use core::result::Result;

use web_sys;
use js_sys;
use wasm_bindgen::JsValue;

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

    // #[allow(unused_variables, unreachable_code)]
    // pub fn spawn<F, T>(self, f: F) -> Result<JoinHandle<T>, Error> {
    //     // Result

    //     Ok(JoinHandle(todo!()))
    // }

    pub fn spawn<F, T>(self, f: F) -> Result<JoinHandle<T>, JsValue>
    where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static {
        let worker: web_sys::Worker = web_sys::Worker::new("./worker.js")?;
        let ptr: *mut F = Box::into_raw(Box::new(f));
        let msg: js_sys::Array = js_sys::Array::new();

        msg.push(&wasm_bindgen::memory());
        msg.push(&JsValue::from(ptr as u32));
        let _: Result<(), JsValue> = worker.post_message(&msg);

        error!("Testing 1");
        todo!();
        let result: T = f();
        error!("Testing 2");

        Ok(JoinHandle {
            result,
            worker
        })
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

pub struct JoinHandle<T> {
    result: T,
    worker: web_sys::Worker
}

impl<T> JoinHandle<T> {
    pub fn is_finished(&self) -> bool {
        return false;
    }
}