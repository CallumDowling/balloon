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

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use glam::{Mat4, Vec3};
use rand::Rng;
use tracing::info;

use crate::{
    component::{camera_component::CameraComponent, transform_component::TransformComponent},
    prefabs::{axis_markers::make_axis_markers, cube111::make_111_cube, teapot::make_teapot},
};

use super::{Scene, SceneCreate};
#[derive(Debug)]
pub struct SceneOne;

//impl SceneType for SceneOne {}

impl SceneCreate<SceneOne> for Scene {
    fn new() -> Arc<RwLock<Scene>> {
        let scene = Self {
            entities_index: 0,
            component_map: HashMap::new(),
        };
        let scene = Arc::new(RwLock::new(scene));
        let mut scene_mutable_lock = scene.write().unwrap();

        let cam = scene_mutable_lock.new_entity();
        let mut cam_transform = TransformComponent::new();
        //cam_transform.transform(Mat4::look_at_rh(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 1.0, 0.0)));

        cam_transform.set_transform(
            Mat4::look_at_lh(
                Vec3::new(0.0, 0.0, -10.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ), //cam_transform.transform * Mat4::from_translation(Vec3::new(0.0, 0.0, -50.0))
               //Mat4::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), PI) * Mat4::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), PI) * (cam_transform.transform *  Mat4::from_translation(Vec3::new(0.0, 0.0, -50.0))).inverse()
        );
        scene_mutable_lock.add_component_to_entity(cam, cam_transform);
        scene_mutable_lock.add_component_to_entity(cam, CameraComponent::new());

        //    Vertex{position:Vector3::new(0.5f64,-0.25f64,0f64), color:Vector3::new(0f64, 0f64, 1f64)},
        //    Vertex{position:Vector3::new(0f64,0.5f64,0f64), color:Vector3::new(0f64, 0f64, 1f64)},
        //    Vertex{position:Vector3::new(0.25f64,-0.1f64,-0.2f64), color:Vector3::new(0f64, 0f64, 1f64)}
        //]));
        drop(scene_mutable_lock);
        let mut rng = rand::thread_rng();
        let cube1 =
            make_111_cube(scene.clone(), crate::prefabs::cube111::CubeType::MORPHER).unwrap();
        let cube2 =
            make_111_cube(scene.clone(), crate::prefabs::cube111::CubeType::MORPHER).unwrap();
        let teapot = make_teapot(
            scene.clone(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 500.0,
            },
            crate::prefabs::cube111::CubeType::MORPHER,
        )
        .unwrap();
        let teapot = make_teapot(
            scene.clone(),
            Vec3 {
                x: 500.0,
                y: 0.0,
                z: 0.0,
            },
            crate::prefabs::cube111::CubeType::MORPHER,
        )
        .unwrap();
        let teapot = make_teapot(
            scene.clone(),
            Vec3 {
                x: 500.0,
                y: 0.0,
                z: 500.0,
            },
            crate::prefabs::cube111::CubeType::ROTATOR,
        )
        .unwrap();
        for x in 0..15 {
            let cube_index =
                make_111_cube(scene.clone(), crate::prefabs::cube111::CubeType::ROTATOR).unwrap();
            info!("3");
            let scene_lock = scene.read().unwrap();
            let transforms = scene_lock
                .get_component_vec::<TransformComponent>()
                .unwrap();
            info!("4");
            let mut transforms = transforms.write().unwrap();
            info!("5");
            if let Some(cube_transform) = transforms[cube_index].as_mut() {
                let (x, y, z) = (
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                );
                cube_transform.set_transform(
                    Mat4::from_translation(Vec3::new(x, y, z)) * cube_transform.transform,
                );
            }
        }
        //let cube3 = make_111_cube(scene.clone(), crate::prefabs::cube111::CubeType::ROTATOR).unwrap();
        let _ = make_axis_markers(scene.clone(), 100.0);

        //info!("Scene: {:#?}", scene_mutable_lock);
        let scene_lock = scene.read().unwrap();
        let transforms = scene_lock
            .get_component_vec::<TransformComponent>()
            .unwrap();
        let mut transforms = transforms.write().unwrap();
        if let Some(cube_transform) = transforms[cube1].as_mut() {
            cube_transform.set_transform(
                Mat4::from_translation(Vec3::new(0.0, 0.0, 5.0)) * cube_transform.transform,
            );
        }
        if let Some(cube_transform) = transforms[cube2].as_mut() {
            cube_transform.set_transform(
                Mat4::from_translation(Vec3::new(0.0, 5.0, 0.0)) * cube_transform.transform,
            );
        }

        drop(transforms);
        drop(scene_lock);

        scene.clone()
    }
}
