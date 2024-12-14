use glam::{Vec3};
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex as VulkanVertex};

#[derive(BufferContents, VulkanVertex, Clone, Debug, Copy)]
#[repr(C)]
pub struct PositionColorNormal {
    #[format(R32G32B32_SFLOAT)]
    pub position: Vec3,
    #[format(R32G32B32_SFLOAT)]
    pub color: Vec3,
    #[format(R32G32B32_SFLOAT)]
    pub normal: Vec3,
}

