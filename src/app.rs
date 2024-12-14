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


use crate::component::camera_component::CameraComponent;
use crate::component::transform_component::TransformComponent;
use crate::scene::SceneCreate;
use crate::scene::{scene_one::SceneOne, Scene};

use crate::system::controller_system::ControllerSystem;
use crate::system::renderer_system::RendererSystem;
use anyhow::Result;
//use nalgebra_glm::{translate, Mat4, Vec3};
use core::f32;
use glam::{Mat4, Vec3};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::Arc;
//use no_deadlocks::prelude::{RwLock};
use std::sync::RwLock;

use std::time::Instant;
use tracing::info;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, MouseScrollDelta, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Clone, Copy)]
pub struct UserEvent;

#[derive(Default, Clone, Copy)]
pub struct Input {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
    q: bool,
    e: bool,
}

pub struct App {
    windows: HashMap<WindowId, Arc<Window>>,
    //entities: Vec<Entity>,
    //Systems
    renderer_system: RendererSystem,
    controller_system: ControllerSystem,
    scenes: Vec<Arc<RwLock<Scene>>>,
    last_new_events_time: Option<Instant>,
    last_window_events_time: Option<Instant>,
    current_input: Input,
}

impl App {
    pub fn new(event_loop: &EventLoop<UserEvent>) -> Result<Self> {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default()
                .with_title("Balloon Engine"))
                .unwrap(),
        );
        let mut windows: HashMap<WindowId, Arc<Window>> = HashMap::new();
        windows.insert(window.id(), Arc::clone(&window));
        //let scene_type = SceneOne;
        //let scene = Scene::<SceneOne>::new();
        //let vals: Vec<Box<MyStruct<dyn Debug>>> = vec![
        //Box::new(MyStruct { foo: 5, bar: 6 }),

        let scene_one = <Scene as SceneCreate<SceneOne>>::new();

        let renderer_system =
            RendererSystem::new(Arc::clone(&window), event_loop, Arc::clone(&scene_one))
                .expect("Err making renderer system");
        let controller_system = ControllerSystem::new();

        let mut scenes: Vec<Arc<RwLock<Scene>>> = vec![];
        scenes.push(scene_one);

        Ok(Self {
            windows,
            renderer_system,
            controller_system,
            scenes,
            last_new_events_time: None,
            last_window_events_time: None,
            current_input: Input::default(),
        })
    }

    fn game_loop(&self) {
        self.controller_system.run(self.scenes[0].clone(), self.current_input);

        let scene = self.scenes[0].read().unwrap();
        if let Some(cameras) = scene.get_component_vec::<CameraComponent>() {
            let mut cameras = cameras.write().unwrap();
            let transforms = scene.get_component_vec::<TransformComponent>().unwrap();
            let mut transforms = transforms.write().unwrap();

            let zip = transforms.iter_mut().zip(cameras.iter_mut());
            let iter =
                zip.filter_map(|(transform, camera)| Some((transform.as_mut()?, camera.as_mut()?)));

            for (transform_component, camera_component) in iter {
                if camera_component.is_active {
                    if self.current_input.a {
                        transform_component.transform_op(|transform| {
                            Mat4::from_translation(Vec3::new(0.2, 0.0, 0.0)) * transform
                        });
                    }
                    if self.current_input.d {
                        transform_component.transform_op(|transform| {
                            Mat4::from_translation(Vec3::new(-0.2, 0.0, 0.0)) * transform
                        });
                    }
                    if self.current_input.s {
                        transform_component.transform_op(|transform| {
                            Mat4::from_translation(Vec3::new(0.0, 0.0, 0.2)) * transform
                        });
                    }
                    if self.current_input.w {
                        transform_component.transform_op(|transform| {
                            Mat4::from_translation(Vec3::new(0.0, 0.0, -0.2)) * transform
                        });
                    }
                    if self.current_input.q {
                        transform_component.transform_op(|transform| {
                            Mat4::from_axis_angle(Vec3::Z, PI / 120.0) * transform
                        });
                    }
                    if self.current_input.e {
                        transform_component.transform_op(|transform| {
                            Mat4::from_axis_angle(Vec3::NEG_Z, PI / 120.0) * transform
                        });
                    }
                }
            }
        }
        //info!("Game loop deb3");
    }
}
impl ApplicationHandler<UserEvent> for App {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match self.last_new_events_time {
            Some(time) => {
                let now = Instant::now();
                let elapsed = now.saturating_duration_since(time);
                println!(
                    "Elapsed:{:?}, fps:{:?}, entities: {}",
                    elapsed,
                    1f64 / elapsed.as_secs_f64(),
                    self.scenes[0].read().unwrap().entities_index
                );
                self.last_new_events_time = Some(now);
            }
            None => {
                let now = Instant::now();
                self.last_new_events_time = Some(now);
            }
        }


        self.game_loop();
        self.renderer_system.redraw();

    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, user_event: UserEvent) {
        // Handle user event.
    }
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match self.last_window_events_time {
            Some(time) => {
                let now = Instant::now();
                let elapsed = now.saturating_duration_since(time);
                self.last_window_events_time = Some(now);
            }
            None => {
                let now = Instant::now();
                self.last_window_events_time = Some(now);
            }
        }
        match event {
            WindowEvent::CloseRequested {} => {
                let _ = self.windows.remove(&window_id);
        }
            WindowEvent::Resized(_) => {
                info!("Resized window");
                self.renderer_system.recreate_swapchain = true;
                return;
            }
            WindowEvent::ActivationTokenDone { serial, token } => (),
            WindowEvent::Moved(physical_position) => (),
            WindowEvent::Destroyed => (),
            WindowEvent::DroppedFile(path_buf) => (),
            WindowEvent::HoveredFile(path_buf) => (),
            WindowEvent::HoveredFileCancelled => (),
            WindowEvent::Focused(_) => (),
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => match event.physical_key {
                winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                    winit::keyboard::KeyCode::KeyT => {}
                    winit::keyboard::KeyCode::KeyA => {
                        self.current_input.a = event.state.is_pressed();
                    }
                    winit::keyboard::KeyCode::KeyD => {
                        self.current_input.d = event.state.is_pressed();
                    }
                    winit::keyboard::KeyCode::KeyS => {
                        self.current_input.s = event.state.is_pressed();
                    }
                    winit::keyboard::KeyCode::KeyW => {
                        self.current_input.w = event.state.is_pressed();
                    }
                    winit::keyboard::KeyCode::KeyQ => {
                        self.current_input.q = event.state.is_pressed();
                    }
                    winit::keyboard::KeyCode::KeyE => {
                        self.current_input.e = event.state.is_pressed();
                    }
                    winit::keyboard::KeyCode::Escape => {
                        std::process::exit(0);
                    }
                    default => {}
                },
                winit::keyboard::PhysicalKey::Unidentified(native_key_code) => todo!(),
            },
            WindowEvent::ModifiersChanged(modifiers) => (),
            WindowEvent::Ime(ime) => (),
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => (),
            WindowEvent::CursorEntered { device_id } => (),
            WindowEvent::CursorLeft { device_id } => (),
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    /*let scene = self.scenes[0].write().unwrap();
                    if let Some(mut camera_components) =
                        scene.borrow_component_vec_mut::<CameraComponent>()
                    {
                        for camera in camera_components.iter_mut() {
                            if let Some(camera) = camera.as_mut() {
                                if camera.is_active {
                                    camera.fovy = (camera.fovy - (0.1 * y)).clamp(0.1, f32::MAX);

                                    self.renderer_system.recreate_swapchain = true;
                                    println!("fov{}", camera.fovy);
                                    //break;
                                }
                            }
                        }
                    }
                    drop(scene);*/
                }
                MouseScrollDelta::PixelDelta(x) => {
                    info!("Pixel delta, {:?}", x);
                }
            },
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => (),
            WindowEvent::PinchGesture {
                device_id,
                delta,
                phase,
            } => (),
            WindowEvent::PanGesture {
                device_id,
                delta,
                phase,
            } => (),
            WindowEvent::DoubleTapGesture { device_id } => (),
            WindowEvent::RotationGesture {
                device_id,
                delta,
                phase,
            } => (),
            WindowEvent::TouchpadPressure {
                device_id,
                pressure,
                stage,
            } => (),
            WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
            } => (),
            WindowEvent::Touch(touch) => (),
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer,
            } => (),
            WindowEvent::ThemeChanged(theme) => (),
            WindowEvent::Occluded(_) => (),
            WindowEvent::RedrawRequested => {
                //self.renderer_system.redraw();
            }
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let scene = self.scenes[0].read().unwrap();
                let cameras = scene.get_component_vec::<CameraComponent>().unwrap();
                let mut cameras = cameras.write().unwrap();
                let transforms = scene.get_component_vec::<TransformComponent>().unwrap();
                let mut transforms = transforms.write().unwrap();

                let zip = transforms.iter_mut().zip(cameras.iter_mut());
                let iter = zip.filter_map(|(transform, camera)| {
                    Some((transform.as_mut()?, camera.as_mut()?))
                });

                for (transform_component, camera) in iter {
                    if camera.is_active {
                        let rot0 = Mat4::from_axis_angle(Vec3::Y, -delta.0 as f32 / 100.0);
                        let rot1 = Mat4::from_axis_angle(Vec3::X, -delta.1 as f32 / 100.0);
                        transform_component.transform_op(|transform| {
                            Mat4::from_axis_angle(Vec3::Y, -delta.0 as f32 / 100.0)
                                * Mat4::from_axis_angle(Vec3::X, -delta.1 as f32 / 100.0)
                                * transform
                        });
                    }
                }
            }
            DeviceEvent::Added => {}
            DeviceEvent::Removed => {}
            DeviceEvent::MouseWheel { delta } => {}
            DeviceEvent::Motion { axis, value } => {}
            DeviceEvent::Button { button, state } => {
                info!("Button event: {}, state: {:?}", button, state);
            }
            DeviceEvent::Key(raw_key_event) => {
                info!("Raw key event: {:?}", raw_key_event);
            }
        }
        //info!("Device event: {:?}", event);

        // Handle device event.
    }

    //fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
    //    //info!("About to wait event");
    //    //event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(Instant::now()+Duration::new(1, 0)));
    //   make_some_tris(self.scenes[0].clone());
    //    make_some_tris(self.scenes[0].clone());
    //    self.renderer_system.redraw();
    //self.counter += 1;
    //}
}

/* fn make_some_tris(scene: Arc<RwLock<Scene>>, iterations: u32) {
    //info!("Make some tris");
    let mut rng = rand::thread_rng();
    //info!("About to lock scene end");
    let mut scene = scene.write().unwrap();
    //info!("About to lock scene end");
    //let last_vertex_index = scene.entities_index;
    let mut last_vertex;
    for x in 0..iterations {
        last_vertex = Vertex {
            position: Vector3::new(
                rng.gen_range(0.0f32..1.0f32),
                rng.gen_range(0.0f32..1.0f32),
                1.0
            ),
            color: Vector3::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
        };

        let ent = scene.new_entity();
        //println!("{}",scene.new_entity());
        scene.add_component_to_entity(ent, TransformComponent::new());
        let new_vertex = Vertex {
            position: Vector3::new(
                last_vertex.position.x - 100.0f32,
                last_vertex.position.y,
                1.0,
            ),
            color: Vector3::new(
                last_vertex.color.x,
                last_vertex.color.y,
                last_vertex.color.z,
            ),
        };
        let new_vertex_1 = Vertex {
            position: Vector3::new(
                last_vertex.position.x - 50f32,
                last_vertex.position.y + 100.0f32,
                1.0,
            ),
            color: Vector3::new(
                last_vertex.color.x,
                last_vertex.color.y,
                last_vertex.color.z,
            ),
        };
        scene.add_component_to_entity(
            ent,
            MeshFilterComponent::new(vec![last_vertex.clone(), new_vertex, new_vertex_1]),
        );
        scene.add_component_to_entity(ent, MeshRendererComponent::new(String::from("default")));
    }
    //info!("Make some tris end");
}
 */
