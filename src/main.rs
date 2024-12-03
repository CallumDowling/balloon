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

#![feature(iterator_try_collect)]
#![feature(slice_iter_mut_as_mut_slice)]
mod app;
mod component;
mod geometry;
mod prefabs;
mod scene;
mod shaders;
mod system;

//mod vulkan_device;
//mod vulkan_instance;
use app::{App, UserEvent};
use tracing::{info, Level};
use winit::event_loop::EventLoop;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    //event_loop.set_control_flow(control_flow);

    //#[allow(deprecated)]
    let mut state = App::new(&event_loop).unwrap();

    let _ = event_loop.run_app(&mut state);
}
