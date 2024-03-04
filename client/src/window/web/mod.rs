#![cfg(target_family = "wasm")]

use web_sys::wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlCanvasElement};

/// Find canvas element on page
pub fn get_canvas() -> Option<HtmlCanvasElement> {
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
