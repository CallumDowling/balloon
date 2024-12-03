use std::sync::{Arc, RwLock};

use crate::{
    component::{
        mesh_filter_component::{IndexedPositionColorNormal, MeshFilterComponent},
        mesh_renderer_component::MeshRendererComponent,
        transform_component::TransformComponent,
    },
    geometry::vertex::PositionColorNormal,
    scene::Scene,
};
use anyhow::Result;
//use nalgebra_glm::Vec3;
use glam::Vec3;
use tracing::info;

//use builder pattern?
pub fn make_axis_markers(scene: Arc<RwLock<Scene>>, magnitude: f32) -> Result<usize> {
    //Front side
    let zero: (f32, f32, f32) = (0.0, 0.0, 0.0);

    let x: (f32, f32, f32) = (magnitude, 0.0, 0.0);
    let y: (f32, f32, f32) = (0.0, magnitude, 0.0);
    let z: (f32, f32, f32) = (0.0, 0.0, magnitude);

    let red: (f32, f32, f32) = (1.0, 0.0, 0.0);
    let green: (f32, f32, f32) = (0.0, 1.0, 0.0);
    let blue: (f32, f32, f32) = (0.0, 0.0, 1.0);

    let offsets = vec![
        (zero, red),
        (x, red),
        (zero, green),
        (y, green),
        (zero, blue),
        (z, blue),
    ];

    let indices = vec![0, 1, 2, 3, 4, 5];

    let verts = offsets
        .iter()
        .map(|((x, y, z), (r, g, b))| PositionColorNormal {
            position: Vec3::new(*x, *y, *z),
            color: Vec3::new(*r, *g, *b),
            normal: Vec3::new(0.0, 0.0, 0.0),
        })
        .collect();

    let verts = IndexedPositionColorNormal {
        verts: verts,
        indices: indices,
    };

    info!("Verts axis markers: {:?}", verts);
    let mut scene_mutable_lock = scene.write().unwrap();
    let ent = scene_mutable_lock.new_entity();
    scene_mutable_lock.add_component_to_entity(
        ent,
        MeshFilterComponent {
            indexed_verts: verts,
        },
    );
    scene_mutable_lock
        .add_component_to_entity(ent, MeshRendererComponent::new(String::from("lines")));
    scene_mutable_lock.add_component_to_entity(ent, TransformComponent::new());
    drop(scene_mutable_lock);
    Ok(ent)
}
