use std::rc::Rc;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

pub(self) use self::super::camera::*;
pub(self) use self::super::geometry::*;
pub(self) use self::super::renderable::*;


#[derive(Clone)]
pub struct Renderer {
    camera: Camera,
    globe: Globe
}

impl Renderer {

    pub fn new(gl: Rc<GL>) -> Self {
        let camera = Camera::new(90.0, 1.0, 1.0, 1000.0);
        let globe = Globe::new(200.0, 40, 30);
        Renderer { camera, globe }
    }

    pub fn init(&mut self, gl: Rc<GL>) -> Result<(), JsValue> {
        self.globe.init(gl.clone());
        Ok(())
    }

    pub fn render(&self, gl: Rc<GL>) -> Result<(), JsValue>{
        let canvas: HtmlCanvasElement = gl.canvas().unwrap().dyn_into()?;

        // Set background color
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // Set the viewport
        gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        // Set options
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.enable(GL::DEPTH_TEST);
        gl.enable(GL::CULL_FACE);
        gl.cull_face(GL::BACK);

        // Draw elements
        self.globe.render(gl.clone(), &self.camera);

        // Unset options
        gl.disable(GL::BLEND);
        gl.disable(GL::DEPTH_TEST);
        gl.disable(GL::CULL_FACE);

        Ok(())
    }

}
