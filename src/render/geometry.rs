use std::rc::Rc;
use std::vec::Vec;
use std::f32::consts::PI;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use js_sys::*;
use nalgebra::{Isometry3, Matrix4, MatrixMN, Vector3, Perspective3, Matrix3, Rotation3};

use crate::render::{Renderable, Camera};
use crate::shader::Shader;
use crate::utils::get_memory_buffer;

static GLOBE_VS: &'static str = include_str!("../shader/globe_vs.glsl");
static GLOBE_FS: &'static str = include_str!("../shader/globe_fs.glsl");


#[derive(Clone)]
pub struct Globe {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    indices: Vec<u16>,
    vao: Option<WebGlVertexArrayObject>,
    shader: Option<Shader>,
    texture: Option<WebGlTexture>
}

impl Globe {
    pub fn new(radius: f32,
               width_segments: u32,
               height_segments: u32
    ) -> Self {

        let width_segments = width_segments.max(3);
        let height_segments = height_segments.max(2);

        let mut positions: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut uvs: Vec<f32> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let mut index: u16 = 0;
        let mut grid: Vec<Vec<u16>> = Vec::new();

        for iy in 0..=height_segments {
            let mut vertices_row: Vec<u16> = Vec::new();

            let v = iy as f32 / height_segments as f32;

            let u_offset = if iy == 0 {
                0.5 / width_segments as f32
            } else if iy == height_segments {
                - 0.5 / width_segments as f32
            } else {
                0.0
            };

            for ix in 0..=width_segments {
                let u = ix as f32 / width_segments as f32;

                let vtx = Vector3::new(
                    -radius * (u * 2.0 * PI).cos() * (v * PI).sin(),
                    radius * (v * PI).cos(),
                    radius * (u * 2.0 * PI).sin() * (v * PI).sin()
                );

                positions.push(vtx.x);
                positions.push(vtx.y);
                positions.push(vtx.z);

                let n = vtx.normalize();
                normals.push(n.x);
                normals.push(n.y);
                normals.push(n.z);

                uvs.push(u + u_offset);
                uvs.push(v);

                vertices_row.push(index);
                index += 1;
            }

            grid.push(vertices_row);
        }

        // indices
        for iy in 0..(height_segments as usize) {
            for ix in 0..(width_segments as usize) {
                let a = grid[iy][ix + 1];
                let b = grid[iy][ix];
                let c = grid[iy + 1][ix];
                let d = grid[iy + 1][ix + 1];

                if iy != 0 {
                    indices.push(a);
                    indices.push(b);
                    indices.push(d);
                }

                if iy != (height_segments as usize - 1) {
                    indices.push(b);
                    indices.push(c);
                    indices.push(d);
                }
            }
        }

        Globe { positions, normals, uvs, indices, vao: None, shader: None, texture: None }
    }

}


impl Renderable for Globe {

    fn init(&mut self, gl: Rc<GL>) {
        self.shader = match Shader::new(&gl, GLOBE_VS, GLOBE_FS) {
            Ok(s) => Some(s),
            Err(error) => { log!("Shader error: {}", error); None }
        };

        let program = &self.shader.as_ref().unwrap().program;

        self.texture = self.load_texture_image(gl.clone(), "/data/earthmap1k.jpg");

        self.vao = gl.create_vertex_array();
        gl.bind_vertex_array(self.vao.as_ref());

        let position_attribute = gl.get_attrib_location(&program, "a_position") as u32;
        let normal_attribute: u32 = gl.get_attrib_location(&program, "a_normal") as u32;
        let uv_attribute: u32 = gl.get_attrib_location(&program, "a_uv") as u32;

        self.vertex_buffer::<f32>(gl.clone(), self.positions.as_slice(),
                                  position_attribute, 3);
        gl.enable_vertex_attrib_array(position_attribute);

        self.vertex_buffer::<f32>(gl.clone(), self.normals.as_slice(),
                                  normal_attribute, 3);
        gl.enable_vertex_attrib_array(normal_attribute);

        self.vertex_buffer::<f32>(gl.clone(), self.uvs.as_slice(),
                                  uv_attribute, 2);
        gl.enable_vertex_attrib_array(uv_attribute);

        self.index_buffer(gl.clone(), self.indices.as_slice());
    }

    fn render(&self, gl: Rc<GL>, camera: &Camera) {
        let shader = self.shader.as_ref().unwrap();
        gl.use_program(Some(&shader.program));

        let model_m = Isometry3::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0)
        );
        let view_m = camera.view();
        let projection_m = camera.projection();
        let model_view_m = model_m * view_m;
        let model_view_rot_m: Rotation3<f32> = nalgebra::convert_unchecked(model_view_m);
        let normal_m = model_view_rot_m.inverse().transpose();

        let proj_uni = shader.get_uniform_location(&gl, "u_projectionMatrix");
        gl.uniform_matrix4fv_with_f32_array(proj_uni.as_ref(), false, projection_m.as_matrix().as_slice());

        let model_view_uni = shader.get_uniform_location(&gl, "u_modelViewMatrix");
        gl.uniform_matrix4fv_with_f32_array(model_view_uni.as_ref(), false, model_view_m.to_homogeneous().as_slice());

        let normal_matrix_uni = shader.get_uniform_location(&gl, "u_normalMatrix");
        gl.uniform_matrix3fv_with_f32_array(normal_matrix_uni.as_ref(), false, normal_m.matrix().as_slice());

        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, self.texture.as_ref());
        let diffuse_texture_uni = shader.get_uniform_location(&gl, "s_diffuseTexture");
        gl.uniform1i(diffuse_texture_uni.as_ref(), 0i32);

        gl.bind_vertex_array(self.vao.as_ref());

        gl.draw_elements_with_i32(GL::TRIANGLES, self.indices.len() as i32, GL::UNSIGNED_SHORT, 0);

        // Unset options
        gl.disable(GL::BLEND);
    }
}
