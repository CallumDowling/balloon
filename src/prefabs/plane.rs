use std::sync::{Arc, RwLock};

use crate::scene::Scene;

use anyhow::Result;

pub fn make_plane(scene: Arc<RwLock<Scene>>, magnitude: f32) -> Result<usize> {
    Ok(1)
}
