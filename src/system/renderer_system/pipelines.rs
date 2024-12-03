//Copyright 2024 Callum Dowling
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
//limitations under the License.
pub mod lines;
pub mod teapot;
pub mod teapot_v2;

use std::sync::Arc;
use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::Device,
    memory::allocator::StandardMemoryAllocator,
    pipeline::graphics::vertex_input::Vertex as VulkanVertex,
    render_pass::RenderPass,
};

pub trait GraphicsPipelineWrapper {
    type T: VulkanVertex;
    type E;
    fn name(&self) -> String;
    fn create(
        &mut self,
        //vertex_input: &Self::T,
        device: Arc<Device>,
        render_pass: Arc<RenderPass>,
        image_extent: [u32; 2],
    );
    fn render(
        &self,
        vertex_input: Vec<Self::T>,
        extra: &Self::E,
        memory_allocator: Arc<StandardMemoryAllocator>,
        descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
        command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    );
}
