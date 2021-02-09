use std::collections::HashMap;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use std::cell::RefCell;


#[derive(Clone)]
pub struct Shader {
    pub program: WebGlProgram,
    vertex_attrs: RefCell<HashMap<String, Option<u32>>>,
    uniforms: RefCell<HashMap<String, Option<WebGlUniformLocation>>>
}

impl Shader {
    pub fn new(
        gl: &GL,
        vert_shader: &str,
        frag_shader: &str
    ) -> Result<Shader, String> {

        let vs = compile_shader(&gl, GL::VERTEX_SHADER, vert_shader)?;
        let fs = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_shader)?;
        let program = link_program(&gl, &vs, &fs)?;

        let vertex_attrs = RefCell::new(HashMap::new());
        let uniforms = RefCell::new(HashMap::new());

        Ok(Shader { program, vertex_attrs, uniforms })
    }

    pub fn get_attrib_location(&self, gl: &GL, name: &str) -> Option<u32> {
        let mut vertex_attrs = self.vertex_attrs.borrow_mut();
        *vertex_attrs.entry(name.to_string())
            .or_insert(match gl.get_attrib_location(&self.program, name) {
                x if x < 0 => None,
                x => Some(x as u32)
            })
    }

    pub fn get_uniform_location(&self, gl: &GL, name: &str) -> Option<WebGlUniformLocation> {
        let mut uniforms = self.uniforms.borrow_mut();
        match uniforms.entry(name.to_string())
            .or_insert(gl.get_uniform_location(&self.program, name)) {
            Some(x) => Some((*x).clone()),
            _ => None
        }
    }
}


fn compile_shader(
    gl: &GL,
    shader_type: u32,
    source: &str
) -> Result<WebGlShader, String> {

    let shader = gl.create_shader(shader_type)
        .ok_or_else(|| "Could not create shader".to_string())?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
    }
}


fn link_program(
    gl: &GL,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader
) -> Result<WebGlProgram, String> {

    let program = gl.create_program()
        .ok_or_else(|| "unable to create shader program".to_string())?;

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
    }
}