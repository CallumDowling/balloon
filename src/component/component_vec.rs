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


use std::fmt::Debug;

use super::Component;

pub trait ComponentVec: std::fmt::Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);

    /* we'll add more functions here in a moment */
}

/*impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>>
where
    T: Debug + Component,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        // `&mut self` already guarantees we have
        // exclusive access to self so can use `get_mut` here
        // which avoids any runtime checks.
        self.get_mut().push(None)
    }
}

impl<T: 'static> ComponentVec for RwLock<Vec<Option<T>>>
where
    T: Debug + Component,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().unwrap().push(None)
    }
}
//<Vec<Option<RwLock<Box<dyn ComponentVec>>>>>>>
impl<T: 'static> ComponentVec for Vec<Option<Arc<RwLock<Box<T>>>>>
where
    T: Debug + Component,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None)
    }
}
*/
//For components where contigious memory is important e.g mesh filter component

impl<T: 'static> ComponentVec for Vec<Option<T>>
where
    T: Debug + Component,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None);
    }
}

/*impl<T: 'static> ComponentVec for RwLock<Vec<Option<T>>>
where
    T: Debug + Component,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.write().unwrap().push(None);
    }
}
*/