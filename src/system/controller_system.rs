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


use std::sync::{Arc, RwLock};

use std::thread;
use rayon::prelude::*;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use crate::{app::Input, component::controller::Controller, scene::{Scene}};


pub struct ControllerSystem {
   
}

impl ControllerSystem {

    pub fn new() -> Self {
      
        Self {}
    }

    pub fn run(&self, scene: Arc<RwLock<Scene>>, current_input: Input) {
        //let mut thread_join_handle;
        let binding = scene.read().unwrap().get_component_vec::<Arc<RwLock<Box<dyn Controller>>>>().unwrap();
        let controllers = binding.read().unwrap().clone();
        
     

        
        
        //for (index, controller) in controllers.into_iter().enumerate() {
        //    if let Some(controller) = controller {
        //        controller.write().unwrap().update(index, scene.clone());
        //    }
        //}

        //let controllers: Vec<Arc<RwLock<Box<dyn Controller+ 'static>>>> = controllers.into_iter().filter_map(|controller: Option<Arc<RwLock<Box<dyn Controller>>>>| Some(controller)?).collect();
        controllers.into_par_iter().enumerate().for_each(|(index, controller)|{
            //let new_scene = scene.clone();
            if let Some(controller) = controller {
                controller.write().unwrap().update(index, scene.clone());
            }
        });


            //if let Some(controller) = controller {
            //    controller.write().unwrap().update(index, scene.clone());
           // }
            /*if let Some(controller) = controller {
                let new_scene = scene.clone();
                thread_join_handle = thread::spawn(move || {
                    let scene = new_scene;
                    controller.write().unwrap().update(index, scene);
                });
                handles.push(thread_join_handle);
            } */
        //}

        //for thread_join_handle in handles {
        //    let _ = thread_join_handle.join();
        //}
        //drop(scene_r_lock);
        //: RwLockWriteGuard<'_,&dyn ComponentVec>
        //let mut map: HashMap<TypeId, Box<dyn Any>> = HashMap::new();
        
        //Creating new components
        
        //let mut scene_w_lock = scene.write().unwrap();
        



        
        /*if let Some(cameras) = controllers_read_lock.get_component_vec::<CameraComponent>() {
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
        }*/
        //info!("Game loop deb3");
    }
}
