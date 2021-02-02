#[macro_use]
mod utils;

mod app;
mod canvas;
mod render;
mod shader;


use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use web_sys::*;
use crate::app::App;
use crate::render::Renderer;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct WebClient {
    app: Rc<App>,
    canvas: Rc<HtmlCanvasElement>,
    gl: Rc<WebGl2RenderingContext>,
    renderer: Rc<Renderer>
}


static APP_ID: &'static str = "rust-webgl";
static WIDTH: u32 = 1024;
static HEIGHT: u32 = 1024;


#[wasm_bindgen]
impl WebClient {
    pub fn new() -> Self {
        utils::set_panic_hook();

        // Create the canvas
        let window = window().unwrap();
        let document = window.document().unwrap();
        let div: HtmlElement = match document.get_element_by_id(APP_ID) {
            Some(container) => container.dyn_into().unwrap(),
            None => {
                let div = document.create_element("div").unwrap();
                div.set_id(APP_ID);
                div.dyn_into().unwrap()
            }
        };
        let canvas = Rc::new(
            canvas::create_canvas(div, WIDTH, HEIGHT).unwrap()
        );

        // Create the WebGl context
        let gl = Rc::new(canvas::create_webgl_context(canvas.clone()).unwrap());

        // Create the Application
        let app = Rc::new(App::new());

        // Create the Renderer
        let renderer = Rc::new(Renderer::new(gl.clone()));

        // Create the WebClient
        WebClient { app, canvas, gl, renderer }
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        Rc::make_mut(&mut self.renderer).init(self.gl.clone())?;
        Ok(())
    }

    pub fn update(&mut self, dt: f64) {
        Rc::make_mut(&mut self.app).update(dt);
    }

    pub fn render(&self) -> Result<(), JsValue> {
        self.renderer.render(self.gl.clone())
    }
}
