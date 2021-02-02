use nalgebra::{Perspective3, Isometry3, Vector3, Point3, Projective3, Matrix4};

#[derive(Clone)]
pub struct Camera {
    view: Isometry3<f32>,
    projection: Perspective3<f32>,
    vfov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32
}

impl Camera {
    pub fn new(vfov:f32, aspect_ratio: f32, near: f32, far:f32) -> Self {
        let position = Point3::new(0.0, 100.0, 250.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&position, &target, &Vector3::y());
        let projection = Perspective3::new(aspect_ratio, vfov, near, far);
        Camera { view, projection, vfov, aspect_ratio, near, far }
    }

    pub fn view(&self) -> &Isometry3<f32> {
        &self.view
    }

    pub fn projection(&self) -> &Perspective3<f32> {
        &self.projection
    }
}
