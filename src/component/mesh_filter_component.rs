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

use crate::geometry::vertex::PositionColorNormal;

//use vulkano::pipeline::graphics::vertex_input::Vertex as VulkanVertex;

use super::Component;
#[derive(Debug, Clone)]
pub struct MeshFilterComponent {
    pub indexed_verts: IndexedPositionColorNormal,
}

#[derive(Debug, Clone)]
pub struct IndexedPositionColorNormal {
    //Goes in vert buffer
    pub verts: Vec<PositionColorNormal>,
    //Goes in index buffer
    pub indices: Vec<u32>,
}

impl Component for MeshFilterComponent {}
