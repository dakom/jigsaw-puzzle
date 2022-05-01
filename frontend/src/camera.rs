use nalgebra_glm::Mat4;
use shipyard::*;

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
pub const Z_DEPTH:f32 = (DEPTH_OFFSET * 125.0) * 2.0; // TODO, make variable by number of pieces
impl Camera {
    pub fn get_matrix(&self, viewport_width: f64, viewport_height: f64) -> Mat4 {

        let left = ((-viewport_width / (2.0 * self.zoom)) + self.x) as f32;
        let right = ((viewport_width / (2.0 * self.zoom)) + self.x) as f32;
        let bottom = ((-viewport_height / (2.0 * self.zoom)) + self.y) as f32;
        let top = ((viewport_height / (2.0 * self.zoom)) + self.y) as f32;

        // change to multiply by n pieces
        Mat4::new_orthographic(left, right, bottom, top, -Z_DEPTH, Z_DEPTH)
    }
}
