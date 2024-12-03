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


use anyhow::{Context, Result};
use glam::Mat4;
use std::sync::Arc;
use vulkano::{
    buffer::{
        allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo},
        Buffer, BufferCreateInfo, BufferUsage,
    },
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{Device},
    memory::allocator::{
        AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator,
    },
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            depth_stencil::{DepthState, DepthStencilState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::{PolygonMode, RasterizationState},
            vertex_input::{Vertex as VulkanVertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::{RenderPass, Subpass},
    shader::EntryPoint,
};

use crate::{geometry::vertex::PositionColorNormal, shaders::vertex};

use super::GraphicsPipelineWrapper;

#[derive(Clone, Debug)]
pub struct LinesExtra {
    pub indices: Vec<u32>,
    pub model: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
}

pub struct Lines {
    pub vs: EntryPoint,
    pub fs: EntryPoint,
    //Set on
    pipeline: Option<Arc<GraphicsPipeline>>,
}
impl Lines {
    pub fn new(device: Arc<Device>) -> Result<Self> {
        let vs = crate::shaders::vertex::load(device.clone())
            .unwrap()
            .entry_point("main")
            .context("Could not create vertex shader")?;

        let fs: vulkano::shader::EntryPoint = crate::shaders::fragment::load(device.clone())
            .unwrap()
            .entry_point("main")
            .context("Could not create frag shader")?;

        Ok(Self {
            vs,
            fs,
            pipeline: None,
        })
    }
}

impl GraphicsPipelineWrapper for Lines {
    type T = PositionColorNormal;
    type E = LinesExtra;

    fn render(
        &self,
        vertex_input: Vec<Self::T>,
        extra: &Self::E,
        memory_allocator: Arc<StandardMemoryAllocator>,
        descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
        command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) {
        if let Some(pipeline) = self.pipeline.clone() {
            //let aspect_ratio = extent[0] as f32 / extent[1] as f32;
            //info!("Input: {:?}", input);
            //info!("Extra: {:?}", extra);
            let vertex_buffer = Buffer::from_iter(
                memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::VERTEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                vertex_input.clone(),
            )
            .unwrap();
            /*let normals_buffer = Buffer::from_iter(
                memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::VERTEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                input.normals.clone(),
            )
            .unwrap();
            */
            let index_buffer = Buffer::from_iter(
                memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::INDEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                extra.indices.clone(),
            )
            .unwrap();

            let uniform_buffer = SubbufferAllocator::new(
                memory_allocator,
                SubbufferAllocatorCreateInfo {
                    buffer_usage: BufferUsage::UNIFORM_BUFFER,
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
            );

            let uniform_buffer_subbuffer = {
                let uniform_data = vertex::VPData {
                    model: extra.model.to_cols_array_2d(),
                    view: extra.view.to_cols_array_2d(),
                    projection: extra.proj.to_cols_array_2d(),
                };

                let subbuffer = uniform_buffer.allocate_sized().unwrap();
                *subbuffer.write().unwrap() = uniform_data;

                subbuffer
            };

            let layout = pipeline.layout().set_layouts().get(0).unwrap();
            let set = PersistentDescriptorSet::new(
                &descriptor_set_allocator,
                layout.clone(),
                [WriteDescriptorSet::buffer(0, uniform_buffer_subbuffer)],
                [],
            )
            .unwrap();

            command_buffer
                .bind_pipeline_graphics(pipeline.clone())
                .unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    pipeline.layout().clone(),
                    0,
                    set,
                )
                .unwrap()
                .bind_vertex_buffers(0, vertex_buffer.clone())
                .unwrap()
                .bind_index_buffer(index_buffer.clone())
                .unwrap()
                .draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
                .unwrap();
        } else {
            panic!("Rendering a pipeline before creation");
        }
    }
    fn create(
        &mut self,
        //input: &Self::T,
        device: Arc<Device>,
        render_pass: Arc<RenderPass>,
        image_extent: [u32; 2],
    ) {
        let pipeline = {
            let vertex_input_state = [Self::T::per_vertex()]
                .definition(&self.vs.info().input_interface)
                .unwrap();
            let stages = [
                PipelineShaderStageCreateInfo::new(self.vs.clone()),
                PipelineShaderStageCreateInfo::new(self.fs.clone()),
            ];
            let layout = PipelineLayout::new(
                device.clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                    .into_pipeline_layout_create_info(device.clone())
                    .unwrap(),
            )
            .unwrap();
            let subpass = Subpass::from(render_pass, 0).unwrap();

            let mut rasterization_state = RasterizationState::default();
            rasterization_state.polygon_mode = PolygonMode::Line;
            //rasterization_state.cull_mode=CullMode::Back;
            rasterization_state.line_width = 1.0f32;

            let mut input_assembly_state = InputAssemblyState::default();
            input_assembly_state.topology =
                vulkano::pipeline::graphics::input_assembly::PrimitiveTopology::LineList;

            GraphicsPipeline::new(
                device,
                None,
                GraphicsPipelineCreateInfo {
                    stages: stages.into_iter().collect(),
                    vertex_input_state: Some(vertex_input_state),
                    input_assembly_state: Some(input_assembly_state),
                    viewport_state: Some(ViewportState {
                        viewports: [Viewport {
                            offset: [0.0, 0.0],
                            extent: [image_extent[0] as f32, image_extent[1] as f32],
                            depth_range: 0.0..=1.0,
                        }]
                        .into_iter()
                        .collect(),
                        ..Default::default()
                    }),
                    rasterization_state: Some(rasterization_state),
                    depth_stencil_state: Some(DepthStencilState {
                        depth: Some(DepthState::simple()),
                        ..Default::default()
                    }),
                    multisample_state: Some(MultisampleState::default()),
                    color_blend_state: Some(ColorBlendState::with_attachment_states(
                        subpass.num_color_attachments(),
                        ColorBlendAttachmentState::default(),
                    )),
                    subpass: Some(subpass.into()),
                    ..GraphicsPipelineCreateInfo::layout(layout)
                },
            )
            .unwrap()
        };
        self.pipeline = Some(pipeline);
    }
    fn name(&self) -> String {
        todo!()
    }
}
