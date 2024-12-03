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


mod pipelines;
use crate::app::UserEvent;
use crate::component::camera_component::CameraComponent;
use crate::component::mesh_filter_component::{MeshFilterComponent};
use crate::component::mesh_renderer_component::{MeshRendererComponent};
use crate::component::transform_component::{TransformComponent};

use crate::scene::Scene;
use itertools::izip;

use anyhow::Result;
use glam::Mat4;
use pipelines::lines::{Lines, LinesExtra};
//use pipelines::lines::{Lines, LinesExtra};
use pipelines::teapot::{Teapot, TeapotExtra};
use pipelines::GraphicsPipelineWrapper;
use vulkano::format::Format;

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::info;
use vulkano::buffer::allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo};
use vulkano::buffer::BufferUsage;
use vulkano::command_buffer::allocator::{CommandBufferAlloc, StandardCommandBufferAllocator};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo,
};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::device::physical::PhysicalDeviceType;
use vulkano::device::{
    Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateInfo, QueueFlags,
};
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{
    AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator,
};
use vulkano::render_pass::{
    Framebuffer, FramebufferCreateInfo, RenderPass,
};
use vulkano::swapchain::{
    acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
};
use vulkano::sync::GpuFuture;
use vulkano::{sync, Validated, Version, VulkanError, VulkanLibrary};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct RendererSystem {
    //attachment_image_views: Vec<Arc<ImageView>>,
    command_buffer_allocator: StandardCommandBufferAllocator,
    device: Arc<Device>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    uniform_buffer_allocator: SubbufferAllocator,
    descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pipelines: HashMap<String, Box<dyn Any>>,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    queue: Arc<Queue>,
    pub recreate_swapchain: bool,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<Image>>,
    current_scene: Arc<RwLock<Scene>>,
    current_window: Arc<Window>,
    first_camera_update: bool, //viewport: Viewport,
    renderpass: Option<Arc<RenderPass>>
}

impl RendererSystem {
    pub fn new(
        window: Arc<Window>,
        event_loop: &EventLoop<UserEvent>,
        scene: Arc<RwLock<Scene>>,
    ) -> Result<Self> {
        let required_extensions = Surface::required_extensions(event_loop);
        let library = VulkanLibrary::new().unwrap();
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                // Enable enumerating devices that use non-conformant Vulkan implementations.
                // (e.g. MoltenVK)
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .unwrap();
        let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

        let mut device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| {
                p.api_version() >= Version::V1_3 || p.supported_extensions().khr_dynamic_rendering
            })
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .expect("no suitable physical device found");

        // Some little debug infos.
        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );
        if physical_device.api_version() < Version::V1_3 {
            device_extensions.khr_dynamic_rendering = true;
        }

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: device_extensions,
                enabled_features: Features {
                    dynamic_rendering: true,
                    fill_mode_non_solid: true,
                    wide_lines: true,
                    ..Features::empty()
                },
                ..Default::default()
            },
        )
        .unwrap();

        let queue = queues.next().unwrap();
        let (swapchain, images) = {
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();

            let image_format: vulkano::format::Format = device
                .physical_device()
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0;

            Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count.max(2),
                    image_format,
                    image_extent: window.inner_size().into(),
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),

                    ..Default::default()
                },
            )
            .unwrap()
        };

        let memory_allocator: Arc<StandardMemoryAllocator> =
            Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        let uniform_buffer_allocator = SubbufferAllocator::new(
            memory_allocator.clone(),
            SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::UNIFORM_BUFFER,
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        );
        //let pipeline_default = pipelines::create_pipeline_default(device.clone(), surface.clone(), swapchain.clone());
        //let vec: Box<dyn GraphicsPipelineWrapper> = vec![];
        let pipeline_teapot = Box::new(Teapot::new(device.clone()).unwrap()) as Box<dyn Any>;
        let pipeline_lines = Box::new(Lines::new(device.clone()).unwrap()) as Box<dyn Any>;
        let mut pipelines = HashMap::new();
        pipelines.insert(String::from("teapot"), pipeline_teapot);
        pipelines.insert(String::from("lines"), pipeline_lines);

        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            Default::default(),
        ));
        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(device.clone(), Default::default());

        let recreate_swapchain = true;
        let previous_frame_end = Some(sync::now(device.clone()).boxed());

        Ok(RendererSystem {
            queue,
            swapchain,
            command_buffer_allocator,
            memory_allocator,
            uniform_buffer_allocator,
            descriptor_set_allocator,
            recreate_swapchain,
            previous_frame_end,
            images,
            pipelines,
            device,
            current_scene: Arc::clone(&scene),
            current_window: Arc::clone(&window),
            first_camera_update: false,
            renderpass: None

        })
    }

    pub fn redraw(&mut self) {
        let image_extent: [u32; 2] = self.current_window.inner_size().into();
        let mut aspect_ratio =
            self.swapchain.image_extent()[0] as f32 / self.swapchain.image_extent()[1] as f32;
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();
        if self.recreate_swapchain {
            let (new_swapchain, new_images) = self
                .swapchain
                .recreate(SwapchainCreateInfo {
                    image_extent,
                    ..self.swapchain.create_info()
                })
                .expect("failed to recreate swapchain");

            self.swapchain = new_swapchain;
            self.images = new_images;
            // Need to update aspect ratio
            aspect_ratio =
                self.swapchain.image_extent()[0] as f32 / self.swapchain.image_extent()[1] as f32;
            self.renderpass =  Some(vulkano::single_pass_renderpass!(
                self.device.clone(),
                attachments: {
                    color: {
                        format: self.swapchain.image_format(),
                        samples: 1,
                        load_op: Clear,
                        store_op: Store,
                    },
                    depth_stencil: {
                        format: Format::D16_UNORM,
                        samples: 1,
                        load_op: Clear,
                        store_op: DontCare,
                    },
                },
                pass: {
                    color: [color],
                    depth_stencil: {depth_stencil},
                },
            )
            .unwrap());    
            for (index, pipeline) in self.pipelines.iter_mut() {
                if let Some(pipeline) = pipeline.downcast_mut::<Teapot>() {
                    info!("Teapot renderer being created");
                    pipeline.create(self.device.clone(), self.renderpass.as_ref().unwrap().clone(), image_extent);
                } else if let Some(pipeline) = pipeline.downcast_mut::<Lines>() {
                    info!("Lines renderer being created");
                    pipeline.create(self.device.clone(), self.renderpass.as_ref().unwrap().clone(), image_extent);
                }
                //Update active camera perspectives
            }
            update_camera_perspective(self.current_scene.clone(), aspect_ratio);

            self.recreate_swapchain = false;
        }
        let (image_index, suboptimal, acquire_future) =
            match acquire_next_image(self.swapchain.clone(), None).map_err(Validated::unwrap) {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("failed to acquire next image: {e}"),
            };
        if suboptimal {
            self.recreate_swapchain = true;
        }

        if !self.first_camera_update {
            update_camera_perspective(self.current_scene.clone(), aspect_ratio);
            self.first_camera_update = true;
        }
      

        //Only need this to include it in framebuffers ???
        let depth_buffer = ImageView::new_default(
            Image::new(
                self.memory_allocator.clone(),
                ImageCreateInfo {
                    image_type: ImageType::Dim2d,
                    format: Format::D16_UNORM,
                    extent: self.images[0].extent(),
                    usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT | ImageUsage::TRANSIENT_ATTACHMENT,
                    ..Default::default()
                },
                AllocationCreateInfo::default(),
            )
            .unwrap(),
        )
        .unwrap();

        let framebuffers = self
            .images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    self.renderpass.as_mut().unwrap().clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view, depth_buffer.clone()],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>();


        let (view, proj) = get_camera_view_and_projection(self.current_scene.clone());

        //let proj = Perspective3::new(image_extent[0] as f32/ image_extent[1] as f32, fovy, znear, zfar);
        //let vertices = generate_vertices(self.current_scene.clone());

        let current_scene = self.current_scene.read().unwrap();

        let binding = current_scene
            .get_component_vec::<TransformComponent>()
            .unwrap();
        //info!("Renderer transform lock");
        let transforms = binding.read().unwrap();

        let binding = current_scene
            .get_component_vec::<MeshFilterComponent>()
            .unwrap();
        let mesh_filters = binding.read().unwrap();

        let binding = current_scene
            .get_component_vec::<MeshRendererComponent>()
            .unwrap();
        let mesh_renderers = binding.read().unwrap();

        //let mut mesh_filters = current_scene.borrow_component_vec_mut::<MeshFilterComponent>().unwrap();
        let zip = izip!(
            transforms.iter(),
            mesh_filters.iter(),
            mesh_renderers.iter()
        );

        let iter = zip.filter_map(|(transform, mesh_filter, mesh_renderer)| {
            Some((
                transform.as_ref()?,
                mesh_filter.as_ref()?,
                mesh_renderer.as_ref()?,
            ))
        });

        let mut builder: AutoCommandBufferBuilder<
            vulkano::command_buffer::PrimaryAutoCommandBuffer,
        > = AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator,
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([1.0, 1.0, 1.0, 1.0].into()), Some(1f32.into())],
                    ..RenderPassBeginInfo::framebuffer(framebuffers[image_index as usize].clone())
                },
                Default::default(),
            )
            .unwrap();

        for (transform_component, mesh_filter_component, mesh_renderer_component) in iter {
            //info!("Object transform: {:?}", transform_component.transform);
            if let Some(target_pipeline) = self.pipelines.get(&mesh_renderer_component.pipeline_key)
            {
                if let Some(renderer) = target_pipeline.downcast_ref::<Teapot>() {
                    renderer.render(
                        mesh_filter_component.indexed_verts.verts.clone(),
                        &TeapotExtra {
                            indices: mesh_filter_component.indexed_verts.indices.clone(),
                            model: transform_component.transform,
                            view: view,
                            proj: proj,
                        },
                        self.memory_allocator.clone(),
                        self.descriptor_set_allocator.clone(),
                        &mut builder,
                    );
                } else if let Some(renderer) = target_pipeline.downcast_ref::<Lines>() {
                    //info!("Here");
                    renderer.render(
                        mesh_filter_component.indexed_verts.verts.clone(),
                        &LinesExtra {
                            indices: mesh_filter_component.indexed_verts.indices.clone(),
                            model: transform_component.transform,
                            view: view,
                            proj: proj,
                        },
                        self.memory_allocator.clone(),
                        self.descriptor_set_allocator.clone(),
                        &mut builder,
                    );
                }

                /*if let Some(mfv) = mesh_filter_component.downcast_ref::<MeshFilterComponent<Vertex>>(){
                    if let Some(renderer) = target.downcast_ref::<VertexRenderer>(){
                        renderer.render(mfv, ());
                    }
                } */
            }
        }
        builder.end_render_pass(Default::default()).unwrap();

        let command_buffer = builder.build().unwrap();
        let future = self
            .previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_index),
            )
            .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(future) => {
                self.previous_frame_end = Some(future.boxed());
            }
            Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("failed to flush future: {e}");
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        }
    }
}

fn update_camera_perspective(current_scene: Arc<RwLock<Scene>>, aspect_ratio: f32) {
    //info!("Camera perspective");
    let current_scene = current_scene.read().unwrap();
    if let Some(binding) = current_scene.get_component_vec::<CameraComponent>() {
        let mut cameras = binding.write().unwrap();

        if let Some(binding) = current_scene.get_component_vec::<TransformComponent>() {
            let transforms = binding.read().unwrap();
            let zip = transforms.iter().zip(cameras.iter_mut());
            let iter =
                zip.filter_map(|(transform, camera)| Some((transform.as_ref()?, camera.as_mut()?)));

            for (_, camera) in iter {
                if camera.is_active {
                    camera.update_perspective(aspect_ratio);
                    break;
                }
            }
        }
    }
}

fn get_camera_view_and_projection(current_scene: Arc<RwLock<Scene>>) -> (Mat4, Mat4) {
    // info!("Camera view and proj");
    let current_scene = current_scene.read().unwrap();
    if let Some(binding) = current_scene.get_component_vec::<CameraComponent>() {
        let cameras = binding.read().unwrap();

        let binding = current_scene
            .get_component_vec::<TransformComponent>()
            .unwrap();
        let mut transforms = binding.write().unwrap();

        let zip = transforms.iter_mut().zip(cameras.iter());
        let iter =
            zip.filter_map(|(transform, camera)| Some((transform.as_mut()?, camera.as_ref()?)));

        for (transform, camera) in iter {
            if camera.is_active {
                //info!("View matrix {:?}",transform.transform);
                if let Some(perspective) = camera.perspective {
                    return (transform.transform, perspective);
                }
            }
        }
    }
    (Mat4::IDENTITY, Mat4::IDENTITY)
}

/*fn generate_vertices(
    current_scene: Arc<RwLock<Scene>>,
) -> HashMap<String, Vec<(Mat4, Vec<Vertex>)>> {
    let current_scene = current_scene.read().unwrap();
    let mut vertices: HashMap<String, Vec<(Mat4, Vec<Vertex>)>> = HashMap::new();
    let Some(mesh_filters) = current_scene.borrow_component_vec::<MeshFilterComponent>() else {
        panic!("mesh_filters missing");
    };
    let Some(transforms) = current_scene.borrow_component_vec::<TransformComponent>() else {
        panic!("transforms missing");
    };
    let Some(mesh_renderers) = current_scene.borrow_component_vec::<MeshRendererComponent>() else {
        panic!("mesh_renderer missing");
    };
    //let mut mesh_filters = current_scene.borrow_component_vec_mut::<MeshFilterComponent>().unwrap();
    let zip = izip!(
        transforms.iter(),
        mesh_filters.iter(),
        mesh_renderers.iter()
    );
    let iter = zip.filter_map(|(transform, mesh_filter, mesh_renderer)| {
        Some((
            transform.as_ref()?,
            mesh_filter.as_ref()?,
            mesh_renderer.as_ref()?,
        ))
    });
    //info!("Scene t0: {:?}",current_scene);

    for (transform, mesh_filter, mesh_render) in iter {
        vertices
            .entry(mesh_render.pipeline_key.clone())
            .and_modify(|verts| verts.push((transform.transform, mesh_filter.vertices.clone())))
            .or_insert(vec![(transform.transform, mesh_filter.vertices.clone())]);

        //vertices.extend(mesh_filter.vertices.clone());
    }
    //info!("verts {:?}", vertices);
    vertices
*/
