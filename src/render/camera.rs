use nalgebra::{Perspective3, Isometry3, Vector3, Point3, Projective3, Matrix4, Transform3};

#[derive(Clone)]
pub struct Camera {
    position: Point3<f32>,
    target: Point3<f32>,
    view: Transform3<f32>,
    projection: Perspective3<f32>,
    vfov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32
}

impl Camera {
    pub fn new(vfov:f32, aspect_ratio: f32, near: f32, far:f32) -> Self {
        let position = Point3::new(0.0, 0.0, -100.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Transform3::from_matrix_unchecked(
            Isometry3::look_at_rh(&position, &target, &Vector3::y()).to_homogeneous()
        );
        let projection = Perspective3::new(aspect_ratio, vfov, near, far);
        Camera { position, target, view, projection, vfov, aspect_ratio, near, far }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Point3::new(x, y, z);
        self.update();
    }

    pub fn set_target(&mut self, x: f32, y: f32, z: f32) {
        self.target = Point3::new(x, y, z);
        self.update();
    }

    pub fn view(&self) -> &Transform3<f32> {
        &self.view
    }

    pub fn projection(&self) -> &Perspective3<f32> {
        &self.projection
    }

    fn update(&mut self) {
        self.view = Transform3::from_matrix_unchecked(
            Isometry3::look_at_rh(
                &self.position,
                &self.target,
                &Vector3::y()
            ).to_homogeneous()
        );
    }
}
