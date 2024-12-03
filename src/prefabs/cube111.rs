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

use crate::{
    component::{
        controller::{
            color_controller::ColorController, rotator_controller::RotatorController, Controller,
        },
        mesh_filter_component::{IndexedPositionColorNormal, MeshFilterComponent},
        mesh_renderer_component::MeshRendererComponent,
        transform_component::TransformComponent,
    },
    geometry::vertex::PositionColorNormal,
    scene::Scene,
};
use anyhow::Result;
use glam::Vec3;

//Use this pattenn for scene
pub enum CubeType {
    MORPHER,
    ROTATOR,
}

//use builder pattern?
pub fn make_111_cube(scene: Arc<RwLock<Scene>>, cube_type: CubeType) -> Result<usize> {
    //let mut scene:  = scene.write().unwrap();
    //Front side
    //let ldf: (f32, f32, f32) = (-0.5,0.5,-0.5);
    //let rdf: (f32, f32, f32) = (0.5, 0.5,-0.5);
    //let ruf: (f32, f32, f32) = (0.5, -0.5,-0.5);
    //let luf: (f32, f32, f32)= (-0.5,-0.5,-0.5);
    //Back side
    //let ldb: (f32, f32, f32) = (-0.5,0.5,0.5);
    //let rdb: (f32, f32, f32) = (0.5, 0.5, 0.5);
    //let rub: (f32, f32, f32) = (0.5, -0.5,0.5);
    //let lub: (f32, f32, f32)= (-0.5,-0.5,0.5);

    //Front side
    let ldf: (f32, f32, f32) = (-0.5, 0.5, -0.5);
    let rdf: (f32, f32, f32) = (0.5, 0.5, -0.5);
    let ruf: (f32, f32, f32) = (0.5, -0.5, -0.5);
    let luf: (f32, f32, f32) = (-0.5, -0.5, -0.5);
    //Back side
    let ldb: (f32, f32, f32) = (-0.5, 0.5, 0.5);
    let rdb: (f32, f32, f32) = (0.5, 0.5, 0.5);
    let rub: (f32, f32, f32) = (0.5, -0.5, 0.5);
    let lub: (f32, f32, f32) = (-0.5, -0.5, 0.5);

    let offsets: Vec<(f32, f32, f32)> = vec![ldf, rdf, ruf, luf, ldb, rdb, rub, lub];

    let indices = vec![
        0, 1, 2, 2, 3, 0, 5, 4, 7, 7, 6, 5, 3, 2, 6, 6, 7, 3, 1, 5, 6, 6, 2, 1, 4, 0, 3, 3, 7, 4,
        4, 5, 1, 1, 0, 4,
    ];

    let verts = offsets
        .iter()
        .map(|(x, y, z)| PositionColorNormal {
            position: Vec3::new(*x, *y, *z),
            color: Vec3::new(0.6 + x, 0.6 + y, 0.6 + z),
            normal: Vec3::new(0.0, 0.0, 0.0),
        })
        .collect();

    let verts = IndexedPositionColorNormal {
        verts: verts,
        indices: indices,
    };

    let mut scene_mutable_lock = scene.write().unwrap();

    let ent = scene_mutable_lock.new_entity();
    scene_mutable_lock.add_component_to_entity(
        ent,
        MeshFilterComponent {
            indexed_verts: verts,
        },
    );
    scene_mutable_lock
        .add_component_to_entity(ent, MeshRendererComponent::new(String::from("teapot")));
    scene_mutable_lock.add_component_to_entity(ent, TransformComponent::new());
    let controller: Box<dyn Controller>;
    match cube_type {
        CubeType::MORPHER => {
            controller = Box::new(ColorController::new());
        }
        CubeType::ROTATOR => {
            controller = Box::new(RotatorController::new());
        }
    }

    //let controller = ColorController::new();

    //let controller: Box<dyn Controller> = controller;

    scene_mutable_lock.add_component_to_entity(ent, Arc::new(RwLock::new(controller)));
    drop(scene_mutable_lock);
    Ok(ent)
}
