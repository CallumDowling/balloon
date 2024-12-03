//use nalgebra::Vector3;
//use nalgebra_glm::Vec3;
use glam::{Mat4, Vec3};
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex as VulkanVertex};

/*#[derive(BufferContents, VulkanVertex, Clone, Debug, Copy)]
#[repr(C)]
pub struct Vertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: Vec3,
}

#[derive(BufferContents, VulkanVertex, Clone, Debug, Copy)]
#[repr(C)]
pub struct VertexColor {
    #[format(R32G32B32_SFLOAT)]
    pub position: Vec3,
    #[format(R32G32B32_SFLOAT)]
    pub color: Vec3,
} */

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

#[derive(BufferContents, VulkanVertex, Clone, Debug, Copy)]
#[repr(C)]
pub struct PositionColorNormalTransform {
    #[format(R32G32B32_SFLOAT)]
    pub position: Vec3,
    #[format(R32G32B32_SFLOAT)]
    pub color: Vec3,
    #[format(R32G32B32_SFLOAT)]
    pub normal: Vec3,
    #[format(R32_UINT)]
    pub transform_index: u32
}

#[derive(Debug, Clone)]
pub struct IndexedPositionColorNormalTransform {
    //Goes in vert buffer
    pub verts: Vec<PositionColorNormalTransform>,
    //Goes in index buffer
    pub indices: Vec<u32>,
    pub transforms: Vec<Mat4>
}