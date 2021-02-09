use std::mem::size_of;
use std::rc::Rc;
use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use crate::utils::get_memory_buffer;
use super::Camera;
use super::Texture;
use crate::shader::Shader;

use std::cell::RefCell;
use wasm_bindgen::__rt::std::collections::HashMap;
use nalgebra::{Isometry3, Rotation3, Similarity3, Transform3};


pub trait Render {
    fn render(&self, gl: &GL, model_matrix: &Transform3<f32>, camera: &Camera);
}


#[derive(Clone)]
pub struct Renderable {
    shader: Rc<Shader>,
    vao: WebGlVertexArrayObject,
    attributes: HashMap<String, u32>,
    num_indices: u32,
    indices_type: u32,
    textures: HashMap<String, Texture>
}

impl Renderable {
    pub fn new(gl: &GL, shader: Rc<Shader>) -> Self {
        let vao = gl.create_vertex_array().unwrap();
        let attributes = HashMap::new();
        let textures = HashMap::new();
        Renderable {
            shader,
            vao,
            attributes,
            num_indices: 0,
            indices_type: GL::UNSIGNED_SHORT,
            textures
        }
    }

    pub fn vertex_attribute<T: CreateArray>(&mut self, gl: &GL, name: &str, data: &[T], size: i32) {
        let attr_location = self.shader.get_attrib_location(&gl, name);
        if attr_location.is_none() {
           log!("Cannot find attribute'{}'", name);
            return
        }

        gl.bind_vertex_array(Some(&self.vao));

        let data_location = data.as_ptr() as u32 / size_of::<T>() as u32;
        let array = T::create_array(data_location, data_location + data.len() as u32);
        let buffer = gl.create_buffer();
        gl.bind_buffer(GL::ARRAY_BUFFER, buffer.as_ref());
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(attr_location.unwrap(), size, T::data_type(), false, 0, 0);

        gl.bind_vertex_array(None);
        self.attributes.insert(name.to_string(), attr_location.unwrap());
    }

    pub fn index_buffer<T: CreateArray>(&mut self, gl: &GL, data: &[T]) {
        gl.bind_vertex_array(Some(&self.vao));

        let data_location = data.as_ptr() as u32 / size_of::<T>() as u32;
        let array = T::create_array(data_location, data_location + data.len() as u32);
        let buffer = gl.create_buffer();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, buffer.as_ref());
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &array, GL::STATIC_DRAW);
        self.num_indices = data.len() as u32;
        self.indices_type = T::data_type();

        gl.bind_vertex_array(None);
    }

    pub fn texture(&mut self, gl: Rc<GL>, src: &str, texture_name: &str) {
        let texture = Texture::new(gl.clone(), src);
        self.textures.insert(texture_name.to_string(), texture);
    }

    fn bind(&self, gl: &GL) {
        gl.use_program(Some(&self.shader.program));
        gl.bind_vertex_array(Some(&self.vao));
        for (name, location) in &self.attributes {
            gl.enable_vertex_attrib_array(*location);
        }
        let mut texture_unit = 0i32;
        for (texture_name, texture) in &self.textures {
            let location = self.shader.get_uniform_location(gl, &texture_name);
            gl.active_texture(GL::TEXTURE0 + texture_unit as u32);
            gl.bind_texture(GL::TEXTURE_2D, Some(texture.get_texture()));
            gl.uniform1i(location.as_ref(), texture_unit);
        }
    }

    fn unbind(&self, gl: &GL) {
        for texture_unit in 0..self.textures.len() {
            gl.active_texture(GL::TEXTURE0 + texture_unit as u32);
            gl.bind_texture(GL::TEXTURE_2D, None);
        }
        for (name, location) in &self.attributes {
            gl.disable_vertex_attrib_array(*location);
        }
        gl.bind_vertex_array(None);
        gl.use_program(None);
    }
}

impl Render for Renderable {
    fn render(&self, gl: &GL, model_matrix: &Transform3<f32>, camera: &Camera) {
        self.bind(gl);

        let view_m = camera.view().to_homogeneous();
        let projection_m = camera.projection();
        let model_view_m = camera.view() * model_matrix;
        let model_view_rot_m: Rotation3<f32> = nalgebra::convert_unchecked(model_view_m);
        let normal_m = model_view_rot_m.inverse().transpose();

        let proj_uni = self.shader.get_uniform_location(gl, "u_projectionMatrix");
        gl.uniform_matrix4fv_with_f32_array(proj_uni.as_ref(), false, projection_m.as_matrix().as_slice());

        let model_view_uni = self.shader.get_uniform_location(gl, "u_modelViewMatrix");
        gl.uniform_matrix4fv_with_f32_array(model_view_uni.as_ref(), false, model_view_m.to_homogeneous().as_slice());

        let normal_matrix_uni = self.shader.get_uniform_location(gl, "u_normalMatrix");
        gl.uniform_matrix3fv_with_f32_array(normal_matrix_uni.as_ref(), false, normal_m.matrix().as_slice());

        gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices as i32, self.indices_type, 0);

        self.unbind(gl);
    }
}


pub trait CreateArray {
    fn create_array(begin: u32, end: u32) -> Object;
    fn data_type() -> u32;
}

impl CreateArray for f32 {
    fn create_array(begin: u32, end: u32) -> Object {
        Float32Array::new(&get_memory_buffer())
            .subarray(begin, end)
            .dyn_into()
            .unwrap()
    }
    fn data_type() -> u32 {
        GL::FLOAT
    }
}

impl CreateArray for u8 {
    fn create_array(begin: u32, end: u32) -> Object {
        Uint8Array::new(&get_memory_buffer())
            .subarray(begin, end)
            .dyn_into()
            .unwrap()
    }
    fn data_type() -> u32 {
        GL::UNSIGNED_BYTE
    }
}

impl CreateArray for u16 {
    fn create_array(begin: u32, end: u32) -> Object {
        Uint16Array::new(&get_memory_buffer())
            .subarray(begin, end)
            .dyn_into()
            .unwrap()
    }
    fn data_type() -> u32 {
        GL::UNSIGNED_SHORT
    }
}

impl CreateArray for u32 {
    fn create_array(begin: u32, end: u32) -> Object {
        Uint32Array::new(&get_memory_buffer())
            .subarray(begin, end)
            .dyn_into()
            .unwrap()
    }
    fn data_type() -> u32 {
        GL::UNSIGNED_INT
    }
}



