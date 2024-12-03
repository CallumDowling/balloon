use glam::Mat4;
use tracing::info;

use super::Component;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct CameraComponent {
    pub fovy: f32,
    pub near: f32,
    pub far: f32,
    pub perspective: Option<Mat4>,
    pub is_active: bool,
}
impl CameraComponent {
    pub fn new() -> Self {
        let fovy = 0.9;
        let near = 1.0;
        let far = 2000.0;
        CameraComponent {
            fovy,
            near,
            far,
            perspective: None,
            is_active: true,
        }
    }
    pub fn update_perspective(&mut self, aspect: f32) {
        info!("Setting perspective, aspect: {}", aspect);
        self.perspective = Some(Mat4::perspective_lh(self.fovy, aspect, self.near, self.far));
    }
}
impl Component for CameraComponent {}
