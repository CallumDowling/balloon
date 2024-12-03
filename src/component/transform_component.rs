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


use super::Component;
use glam::{Mat4, Vec3, Vec4Swizzles};

#[derive(Debug)]
pub struct TransformComponent {
    pub transform: Mat4,
    parent: Option<usize>,
    //scene: Arc<RwLock<Scene>>,
    //dirty: bool
}


impl TransformComponent {
    pub fn new() -> Self {
        let transform = Mat4::IDENTITY;
        let parent: Option<usize> = None;

        TransformComponent {
            transform,
            parent,
            //scene,
            //dirty: true
        }
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform;
    }

    pub fn transform_op(&mut self, op: impl Fn(Mat4) -> Mat4) {
        self.transform = op(self.transform);
        if let Some(parent) = self.parent {}
    }

    //Seems a bit sketch
    pub fn forward(&self) -> Vec3 {
        self.transform.row(3).xyz()
    }
}

impl Component for TransformComponent {}
