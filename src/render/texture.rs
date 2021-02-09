use std::rc::Rc;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


#[derive(Clone)]
pub struct Texture {
    texture: WebGlTexture
}

impl Texture {
    pub fn new(gl: Rc<GL>, src: &str) -> Texture {
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

        gl.bind_texture(GL::TEXTURE_2D, None);

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
            gl.bind_texture(GL::TEXTURE_2D, None);
        }) as Box<dyn Fn()>);

        image.set_onload(Some(onload.as_ref().unchecked_ref()));
        image.set_src(src);

        onload.forget();

        Texture { texture: texture.unwrap() }
    }

    pub fn get_texture(&self) -> &WebGlTexture {
        &self.texture
    }
}




