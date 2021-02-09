use std::rc::Rc;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;
use wasm_bindgen::JsCast;

use crate::render::{Camera, Globe, Renderer, Render};

//#[derive(Clone)]
pub struct App {
    time: f64,
    camera: Camera,
    renderables: Vec<Box<dyn Render>>
}

impl App {
    pub fn new(gl: Rc<GL>) -> Self {
        let canvas: HtmlCanvasElement = gl.canvas().unwrap().dyn_into().unwrap();
        let w = canvas.width() as f32;
        let h = canvas.height() as f32;
        let mut camera = Camera::new(30.0, w / h, 1.0, 1000.0);
        camera.set_position(0.0, 0.0, 500.0);
        camera.set_target(0.0, 0.0, 0.0);
        let globe = Globe::new(gl.clone(), 200.0, 40, 30);

        App {
            time: 0.0,
            camera,
            renderables: vec![Box::new(globe)]
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt;
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_renderables(&self) -> &[Box<dyn Render>] {
        self.renderables.as_slice()
    }
}
