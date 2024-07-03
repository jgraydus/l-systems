use wasm_bindgen::prelude::*;
use web_sys::{
    console,
    MouseEvent,
    WheelEvent,
};

pub fn set_up_mouse_handlers() -> () {
    let handle_mousedown = {
        Closure::<dyn FnMut(MouseEvent)>::new(move |evt: MouseEvent| {
            // TODO
        })
    };
    let handle_mouseup = {
        Closure::<dyn FnMut(MouseEvent)>::new(move |evt: MouseEvent| {
            // TODO
        })
    };
    let handle_wheel = {
        Closure::<dyn FnMut(WheelEvent)>::new(move |evt: WheelEvent| {
            // TODO
            console::log_1(&format!("delta_x: {}, delta_y: {}", evt.delta_x(), evt.delta_y()).into());
        })
    };

    let window = web_sys::window().expect("no window?!");
    window.set_onmousedown(Some(handle_mousedown.as_ref().unchecked_ref()));
    handle_mousedown.forget();
    window.set_onmouseup(Some(handle_mouseup.as_ref().unchecked_ref()));
    handle_mouseup.forget();
    window.set_onwheel(Some(handle_wheel.as_ref().unchecked_ref()));
    handle_wheel.forget();
}
