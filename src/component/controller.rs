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


use crate::scene::{Scene};

use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use super::Component;

pub mod color_controller;
pub mod rotator_controller;

pub trait Controller : Send + Sync {
    fn update(&mut self, index: usize, scene: Arc<RwLock<Scene>>);

    fn fixed_update(&self, _: usize, _: Scene) {}
    fn late_update(&self, _: usize, _: Scene) {}
}

//Implementing component for all structs that impl controller
//impl<T> Component for T
//where
//    T: Controller + Debug,
//{

//}

//impl<T> Component for T where T: Controller + Debug + Sized {}

impl Component for Arc<RwLock<Box<dyn Controller>>>{}
//impl Component for Box<dyn Controller> {}




impl Debug for dyn Controller {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Controller, output coming soon!")
    }
}
