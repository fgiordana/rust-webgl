use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;


pub fn create_canvas(parent: HtmlElement, width: u32, height: u32) -> Result<HtmlCanvasElement, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    canvas.set_width(width);
    canvas.set_height(height);

    parent.append_child(&canvas)?;

    Ok(canvas)
}


pub fn create_webgl_context(canvas: Rc<HtmlCanvasElement>) -> Result<WebGl2RenderingContext, JsValue> {
    let gl: WebGl2RenderingContext = canvas.get_context("webgl2")?.unwrap().dyn_into()?;
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.enable(GL::DEPTH_TEST);

    Ok(gl)
}

