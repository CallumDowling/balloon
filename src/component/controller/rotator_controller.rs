use std::sync::{Arc, RwLock};
use std::time::Instant;

use glam::{Mat4, Vec3};
use rand::Rng;
use tracing::info;
use std::fmt::Debug;

use crate::component::transform_component::TransformComponent;
use crate::prefabs::cube111::make_111_cube;
use crate::scene::Scene;


use super::Controller;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RotatorController {
    last_update: Instant,
}
impl RotatorController {
    pub fn new() -> Self {
        RotatorController {
            last_update: Instant::now(),
        }
    }
}

impl Controller for RotatorController {
    fn update(&mut self, index: usize, scene: Arc<RwLock<Scene>>) {
        let scene_lock = scene.read().unwrap();
        let mut rng = rand::thread_rng();
        let now = Instant::now();
        let elapsed = now.saturating_duration_since(self.last_update);

        let transform_components = scene_lock
            .get_component_vec::<TransformComponent>()
            .unwrap();
        let mut transform_components = transform_components.write().unwrap();
        if let Some(transform_component) = transform_components[index].as_mut() {
            let rot1 = Mat4::from_axis_angle(Vec3::X, 0.02);
            transform_component.transform_op(|transform| transform * rot1);
        }
        /*let mut  event_batch = vec![];
        event_batch.push(
            SceneEvents {
                entity: Some(index),
                events: vec![
                    SceneEvent::ComponentOp(ComponentOp::Modify(Modify::TransformOp(Box::new(|transform| transform * Mat4::from_axis_angle(Vec3::X, 0.02)))))
                ]
            }
        );
        event_batch
    
        drop(transform_components);
        drop(scene_lock);
        if elapsed.as_secs() > 2 {
            info!("Cube made prior");
            let cube = make_111_cube(scene.clone(), crate::prefabs::cube111::CubeType::MORPHER).unwrap();
            info!("Cube made");
            let scene_lock = scene.read().unwrap();
            info!("Cube Scene lock");
            let transform_components = scene_lock.get_component_vec::<TransformComponent>().unwrap();
            let mut transform_components = transform_components.write().unwrap();
            let (x,y,z) = (rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));
            transform_components[cube].as_mut().unwrap().transform_op(|transform| transform * Mat4::from_translation(Vec3{x: x, y: y, z: z}));
            self.last_update = now;
            
        }
        */
        //info!("End of update");
    }
}
