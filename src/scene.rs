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


//use no_deadlocks::prelude::{RwLock};
use std::sync::Arc;
use std::sync::RwLock;
use crate::component::component_vec::ComponentVec;
use crate::component::Component;
use std::fmt::Debug;
pub mod scene_one;
use std::{any::TypeId, collections::HashMap};

const THRESHOLD: usize = 50000;

pub trait SceneCreate<T> {
    fn new() -> Arc<RwLock<Scene>>;
}

#[derive(Debug)]
pub struct Scene {
    pub entities_index: usize,
    pub component_map: HashMap<TypeId, Arc<RwLock<dyn ComponentVec + Send + Sync>>>,
}


impl Scene {
    /*pub fn update(&mut self){
        for controller_componenet in self.get_component_vec::<Controller>() {

        }
    }
    pub fn process_scene_events(&mut self, mut scene_events:Vec<SceneEvents>) {
        let test = SceneEvent::TransformOp(ComponentOp::Delete);





        let binding = self.get_component_vec::<Box<dyn Controller>>().unwrap();
        let mut controller_component_w:RwLockWriteGuard<'_, Vec<Option<Box<dyn Controller>>>> = binding.write().unwrap();
        let binding = self.get_component_vec::<CameraComponent>().unwrap();
        let mut camera_component_w:RwLockWriteGuard<'_, Vec<Option<CameraComponent>>> = binding.write().unwrap();
        let binding = self.get_component_vec::<MeshFilterComponent>().unwrap();
        let mut mesh_filter_component_w:RwLockWriteGuard<'_, Vec<Option<MeshFilterComponent>>> = binding.write().unwrap();
        let binding = self.get_component_vec::<MeshRendererComponent>().unwrap();
        let mut mesh_renderer_component_w:RwLockWriteGuard<'_, Vec<Option<MeshRendererComponent>>> = binding.write().unwrap();
        let binding = self.get_component_vec::<TransformComponent>().unwrap();
        let mut transform_component_w:RwLockWriteGuard<'_, Vec<Option<TransformComponent>>> = binding.write().unwrap();
        

        //Creating new component where entity id is left blank
        //let mut scene_w_lock = scene.write().unwrap();
        for controller_system_events in &mut scene_events {
            if let None = controller_system_events.entity {
                controller_system_events.entity= Some(self.new_entity());
            } 
        }

        for controller_system_events in scene_events {
            //All event batches should have an index at this stage
            let index = controller_system_events.entity.unwrap();
            
            for controller_system_event in controller_system_events.events{
                match controller_system_event {
                    SceneEvent::DeleteEntity => todo!(),
                    SceneEvent::CameraOp(component_op) => todo!(),
                    SceneEvent::TransformOp(component_op) => {
                        match component_op {
                            ComponentOp::Set(_) => todo!(),
                            ComponentOp::Delete => todo!(),
                            ComponentOp::Modify(modify_op) => {
                                match modify_op {
                                    ModifyTransform => {

                                    }
                                }
                            },
                        }},
                    SceneEvent::MeshFilterOp(component_op) => todo!(),
                    SceneEvent::MeshRendererOp(component_op) => todo!(),
                    SceneEvent::ControllerOp(component_op) => todo!(),
                    
                    SceneEvent::DeleteEntity => todo!(),
                    
                }
            }
        }
    }*/
    
    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_index;
        //info!("New entitiy");

        for (key, value) in self.component_map.iter_mut() {
            //let mut value = value.write().unwrap()
            value.write().unwrap().push_none();
        }
        self.entities_index += 1;
        entity_id
    }

    #[track_caller]
    pub fn get_component_vec<ComponentType: 'static + Component + Send + Sync>(
        &self,
    ) -> Option<Arc<RwLock<Vec<Option<ComponentType>>>>> {
        let caller_location = std::panic::Location::caller();
        let caller_line_number = caller_location.line();
        let caller_file = caller_location.file();
        //info!("get component, typeid {:?}, is transform component: {:?}, Called from: {}{}",&TypeId::of::<ComponentType>(), &TypeId::of::<ComponentType>() == &TypeId::of::<crate::component::transform_component::TransformComponent>(),caller_file, caller_line_number);
        if let Some(component_vec_1) = self.component_map.get(&TypeId::of::<ComponentType>()) {
            //let test = *component_vec_1;
            //info!("Deb prior to downcast ref");
            let s = unsafe {
                std::mem::transmute::<
                    &Arc<RwLock<dyn ComponentVec + Send + Sync>>,
                    &Arc<RwLock<Vec<Option<ComponentType>>>>,
                >(component_vec_1)
            };
            //std::mem::forget(component_vec_1);
            //info!("Deb2");
            return Some(s.clone());
            /*if let Some(component_vec_2) = component_vec_1
                .read()
                .unwrap()
                .as_any()
                .downcast_ref::<Vec<Option<ComponentType>>>()
            {
                info!("Downcast ref true");
                let s = unsafe { std::mem::transmute::<&Arc<RwLock<dyn ComponentVec>>, &Arc<RwLock<Vec<Option<ComponentType>>>>>(component_vec_1) };
                //std::mem::forget(component_vec_1);
                info!("Deb2");
                return Some(s.clone());
                //return std::mem::transmute::<Arc<dyn ComponentVec>, Option<Arc<RwLock<Vec<Option<ComponentType>>>>>>(*component_vec_1);
            }*/
        }
        //info!("Deb2");
        None
    }

    pub fn add_component_to_entity<ComponentType: 'static + Send + Sync>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) where
        ComponentType: Component,
    {
        if let Some(component_vec) = self.component_map.get(&TypeId::of::<ComponentType>()) {
            if let Some(component_vec) = component_vec
                .write()
                .unwrap()
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                //info!("Existing component {:?}", std::any::type_name::<ComponentType>());
                //let mut component_vec = component_vec.write().unwrap();
                component_vec[entity] = Some(component);
                return;
            }
        }

        //info!("New component {:?}", std::any::type_name::<ComponentType>());
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_index);

        for _ in 0..self.entities_index {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);

        self.component_map.insert(
            TypeId::of::<ComponentType>(),
            Arc::new(RwLock::new(new_component_vec)),
        );
    }
}
