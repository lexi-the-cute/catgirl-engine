#![cfg(target_family = "wasm")]

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlCanvasElement};

/// Find canvas element on page
pub(crate) fn get_canvas() -> Option<HtmlCanvasElement> {
    let document: Document = web_sys::window()
        .expect("Could not find browser window...")
        .document()
        .expect("Could not find window document...");

    let canvas: Element = web_sys::Document::get_element_by_id(&document, "catgirl-engine-canvas")
        .expect("Could not find canvas element...");

    let canvas_element: HtmlCanvasElement =
        Element::dyn_into(canvas).expect("Could not cast canvas Element to HtmlCanvasElement...");

    Some(canvas_element)
}

/// Prints to log with formatted text
fn print_formatted_test() {
    use wasm_bindgen::JsValue;
    use web_sys::console;

    let message: JsValue = JsValue::from_str("%cTest");
    let style: JsValue = JsValue::from_str("color: orange; font-weight: bold; font-size: 200%");
    console::debug_2(&message, &style);
}
