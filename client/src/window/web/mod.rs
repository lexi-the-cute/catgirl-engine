#![cfg(target_family = "wasm")]

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlCanvasElement};

/// Find canvas element on page
pub(crate) fn get_canvas() -> Option<HtmlCanvasElement> {
    let document: Document = web_sys::window()
        .expect("Could not find browser window...")
        .document()
        .expect("Could not find window document...");

    let mut canvas_option: Option<Element> =
        web_sys::Document::get_element_by_id(&document, "catgirl-engine-canvas");

    // Itch.io's https://html-classic.itch.zone/html/xxxxxxx/index.html uses the id `canvas`
    if canvas_option.is_none() {
        canvas_option = web_sys::Document::get_element_by_id(&document, "canvas");
    }

    let canvas: Element = canvas_option.unwrap();
    let canvas_element: HtmlCanvasElement =
        Element::dyn_into(canvas).expect("Could not cast canvas Element to HtmlCanvasElement...");

    Some(canvas_element)
}
