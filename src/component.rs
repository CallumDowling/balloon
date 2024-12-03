pub mod camera_component;
pub mod component_vec;
pub mod controller;
pub mod mesh_filter_component;
pub mod mesh_renderer_component;
pub mod transform_component;

pub trait Component: std::fmt::Debug + Send {}


//pub struct ComponentHolder<T: Component> {}