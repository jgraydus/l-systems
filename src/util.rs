use web_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

pub fn get_canvas() -> web_sys::HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas
}

pub fn get_context2d() -> web_sys::CanvasRenderingContext2d {
    get_canvas()
        .get_context("2d").unwrap().unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()
}

pub fn clear_canvas(context: &web_sys::CanvasRenderingContext2d) {
    let canvas = context.canvas().expect("canvas is missing");
    let (width, height) = (canvas.width() as f64, canvas.height() as f64);
    context.begin_path();
    context.set_fill_style(&JsValue::from_str("black"));
    context.rect(0.0, 0.0, width, height);
    context.fill();
}

