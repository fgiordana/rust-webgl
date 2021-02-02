use std::mem::size_of;
use std::rc::Rc;
use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use crate::utils::get_memory_buffer;
use crate::render::Camera;


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


pub trait Renderable {
    fn vertex_buffer<T: CreateArray>(&self, gl: Rc<GL>, data: &[T], attrib: u32, size: i32) {
        let data_location = data.as_ptr() as u32 / size_of::<T>() as u32;
        let array = T::create_array(data_location, data_location + data.len() as u32);
        let buffer = gl.create_buffer();
        gl.bind_buffer(GL::ARRAY_BUFFER, buffer.as_ref());
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(attrib, size, T::data_type(), false, 0, 0);
    }

    fn index_buffer<T: CreateArray>(&self, gl: Rc<GL>, data: &[T]) {
        let data_location = data.as_ptr() as u32 / size_of::<T>() as u32;
        let array = T::create_array(data_location, data_location + data.len() as u32);
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    fn load_texture_image(&self, gl: Rc<GL>, src: &str) -> Option<WebGlTexture> {
        let texture = gl.create_texture();
        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            1,
            1,
            0,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            &[0u8, 255u8, 0u8, 255u8],
            0
        );

        let image = Rc::new(HtmlImageElement::new().unwrap());

        let gl_cloned = gl.clone();
        let image_clone = image.clone();
        let texture_clone = texture.clone();

        let onload = Closure::wrap(Box::new(move || {
            gl_cloned.bind_texture(GL::TEXTURE_2D, texture_clone.as_ref());
            gl_cloned.tex_image_2d_with_u32_and_u32_and_html_image_element(
                GL::TEXTURE_2D,
                0,
                GL::RGBA as i32,
                GL::RGBA,
                GL::UNSIGNED_BYTE,
                &image_clone
            );
            gl_cloned.generate_mipmap(GL::TEXTURE_2D);
        }) as Box<dyn Fn()>);

        image.set_onload(Some(onload.as_ref().unchecked_ref()));
        image.set_src(src);

        onload.forget();

        texture
    }

    fn init(&mut self, gl: Rc<GL>);
    fn render(&self, gl: Rc<GL>, camera: &Camera);
}


