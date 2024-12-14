use std::sync::{Arc, RwLock};
use std::time::Instant;

use glam::{Mat4, Vec3};
use rand::Rng;
use std::fmt::Debug;

use crate::component::mesh_filter_component::MeshFilterComponent;
use crate::component::transform_component::TransformComponent;
use crate::scene::Scene;

use super::Controller;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ColorController {
    last_update: Instant,
}
impl ColorController {
    pub fn new() -> Self {
        ColorController {
            last_update: Instant::now(),
        }
    }
}

impl Controller for ColorController {
    fn update(&mut self, index: usize, scene: Arc<RwLock<Scene>>) {
        //info!("Color update");
        let scene = scene.read().unwrap();
        let mut rng = rand::thread_rng();
        let now = Instant::now();
        let elapsed = now.saturating_duration_since(self.last_update);
        //println!("Prior to lock");
        //println!("After lock");
        if elapsed.as_secs_f64() > 0.01 {
            //println!("Elapsed > 2");
             
            let mesh_filter_components = scene.get_component_vec::<MeshFilterComponent>().unwrap();
            let mut mesh_filter_components = mesh_filter_components.write().unwrap();

            if let Some(my_mesh_filter) = mesh_filter_components[index].as_mut() {
                for vertex in &mut my_mesh_filter.indexed_verts.verts {
                    vertex.color = Vec3 {
                        x: rng.gen_range(0..=1) as f32,
                        y: rng.gen_range(0..1) as f32,
                        z: rng.gen_range(0..1) as f32,
                    };
                    vertex.position = Vec3 {
                        x: vertex.position.x + rng.gen_range(-0.01..0.01),
                        y: vertex.position.y + rng.gen_range(-0.01..0.01),
                        z: vertex.position.z + rng.gen_range(-0.01..0.01),
                    }
                }
                self.last_update = now;
            }
            let transform_components = scene.get_component_vec::<TransformComponent>().unwrap();

            let mut transform_components = transform_components.write().unwrap();
            if let Some(transform_component) = transform_components[index].as_mut() {
                let rot1 = Mat4::from_axis_angle(Vec3::Y, 0.02);
                transform_component.transform_op(|transform| transform * rot1);
            }
            //let (x,y,z,i,j,k)= (rng.gen_range(0.2..1.0),rng.gen_range(0.2..1.0),rng.gen_range(0.2..1.0), rng.gen_range(-0.1..0.1),rng.gen_range(-0.1..0.1),rng.gen_range(-0.1..0.1));
           /*  event_batch.push(
                SceneEvents {
                    entity: Some(index),
                    events: vec![
                        //SceneEvent::ComponentOp(ComponentOp::Modify(Modify::TransformOp(Box::new(|transform|  Mat4::from_axis_angle(Vec3::Y, 0.02) * transform)))),
                        SceneEvent::ComponentOp(ComponentOp::Modify(Modify::MeshFilterOp(Box::new(
                            move |indexed_position_normal|
                                for vertex in &mut indexed_position_normal.verts{
                                    let (x,y,z,i,j,k)= (rng.gen_range(0.2..1.0),rng.gen_range(0.2..1.0),rng.gen_range(0.2..1.0), rng.gen_range(-0.1..0.1),rng.gen_range(-0.1..0.1),rng.gen_range(-0.1..0.1));
                                    vertex.color = Vec3 {
                                        x: x,
                                        y: y,
                                        z: z,
                                    };
                                    //info!("NEW POSITIONS x: {}, y: {}, z: {}", vertex.position.x + i, vertex.position.y + j,vertex.position.z + k);
                                    vertex.position.x += i;
                                    vertex.position.y += j;
                                    vertex.position.z += k;
                                }
                        )))),
                        SceneEvent::ComponentOp(ComponentOp::Modify(Modify::TransformOp(Box::new(|transform| transform * Mat4::from_axis_angle(Vec3::Y, 0.02)))))
                    ]
                }
            );*/
            self.last_update = now;
        }
    }
}
