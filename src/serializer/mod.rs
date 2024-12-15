mod deserializer_impl;
mod detail;
mod serializer_impl;

use std::path::Path;

pub use deserializer_impl::Deserializer;
pub use serializer_impl::Serializer;

pub struct CameraData {
    pub location: [f32; 3],
    pub look_at: [f32; 3],
    pub angle: f32,
}

pub trait ISceneProvider {
    fn includes(&self) -> impl Iterator<Item = String>;

    fn camera(&self) -> Option<CameraData>;
}

pub trait ISceneBuilder {
    fn add_include_path<T>(&mut self, path: T)
    where
        T: AsRef<Path>;

    fn set_camera(&mut self, camera_data: CameraData);
}
