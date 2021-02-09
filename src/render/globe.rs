use std::rc::Rc;
use std::vec::Vec;
use std::f32::consts::PI;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use js_sys::*;
use nalgebra::{Isometry3, Matrix4, MatrixMN, Vector3, Perspective3, Matrix3, Rotation3, Transform3, Similarity3};

use crate::render::{Render, Camera, Renderable};
use crate::shader::Shader;
use crate::utils::get_memory_buffer;

static GLOBE_ATMOSPHERE_VS: &'static str = include_str!("../shader/globe_atmosphere_vs.glsl");
static GLOBE_ATMOSPHERE_FS: &'static str = include_str!("../shader/globe_atmosphere_fs.glsl");
static GLOBE_EARTH_VS: &'static str = include_str!("../shader/globe_earth_vs.glsl");
static GLOBE_EARTH_FS: &'static str = include_str!("../shader/globe_earth_fs.glsl");


#[derive(Clone)]
pub struct Globe {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    indices: Vec<u16>,
    earth: Renderable,
    atmosphere: Renderable,
    earth_xform: Transform3<f32>,
    atmosphere_xform: Transform3<f32>
}

impl Globe {
    pub fn new(gl: Rc<GL>,
               radius: f32,
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

        // Earth
        let mut earth = Renderable::new(
            gl.as_ref(),
            Rc::new(Shader::new(
                gl.as_ref(),
                GLOBE_EARTH_VS,
                GLOBE_EARTH_FS
            ).unwrap())
        );
        earth.vertex_attribute(gl.as_ref(), "a_position", positions.as_slice(), 3);
        earth.vertex_attribute(gl.as_ref(), "a_normal", normals.as_slice(), 3);
        earth.vertex_attribute(gl.as_ref(), "a_uv", uvs.as_slice(), 2);
        earth.index_buffer(gl.as_ref(), indices.as_slice());
        earth.texture(gl.clone(), "/data/world.jpg", "s_texture");

        let earth_xform = Transform3::identity();

        // Atmosphere
        let mut atmosphere = Renderable::new(
            gl.as_ref(),
            Rc::new(Shader::new(
                gl.as_ref(),
                GLOBE_ATMOSPHERE_VS,
                GLOBE_ATMOSPHERE_FS
            ).unwrap())
        );
        atmosphere.vertex_attribute(gl.as_ref(), "a_position", positions.as_slice(), 3);
        atmosphere.vertex_attribute(gl.as_ref(), "a_normal", normals.as_slice(), 3);
        atmosphere.index_buffer(gl.as_ref(), indices.as_slice());

        let atmosphere_xform = Transform3::from_matrix_unchecked(
            Similarity3::new(
                Vector3::new(0.0f32, 0.0, 0.0),
                Vector3::new(0.0f32, 0.0, 0.0),
                1.04f32
            ).to_homogeneous()
        );

        Globe {
            positions,
            normals,
            uvs,
            indices,
            earth,
            atmosphere,
            earth_xform,
            atmosphere_xform
        }
    }

}


impl Render for Globe {
    fn render(&self, gl: &GL, model_matrix: &Transform3<f32>, camera: &Camera) {
        self.earth.render(gl, model_matrix, camera);

        let atm_model_matrix = self.atmosphere_xform * model_matrix;
        self.atmosphere.render(gl, &atm_model_matrix, camera);
    }
}
