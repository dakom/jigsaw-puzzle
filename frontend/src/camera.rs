use nalgebra_glm::Mat4;
use shipyard::*;
use crate::dom::Dom;

#[derive(Component)]
pub struct Camera {
    pub zoom: f64,
    pub x: f64,
    pub y: f64
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            zoom: 0.4,
            x: 0.0,
            y: 0.0
        }
    }
}

pub const DEPTH_OFFSET:f32 = 100.0;

impl Camera {
    pub fn get_matrix(&self, viewport_width: f64, viewport_height: f64) -> Mat4 {

        let left = ((-viewport_width / (2.0 * self.zoom)) + self.x) as f32;
        let right = ((viewport_width / (2.0 * self.zoom)) + self.x) as f32;
        let bottom = ((-viewport_height / (2.0 * self.zoom)) + self.y) as f32;
        let top = ((viewport_height / (2.0 * self.zoom)) + self.y) as f32;

        // change to multiply by n pieces
        let z_depth = DEPTH_OFFSET * 125.0;
        Mat4::new_orthographic(left, right, bottom, top, z_depth * -2.0, z_depth * 2.0)
    }
}
