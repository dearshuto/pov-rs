use crate::serializer::ISceneProvider;

pub struct Scene {
    include_files: Vec<String>,
}

impl ISceneProvider for Scene {
    fn includes(&self) -> impl Iterator<Item = String> {
        self.include_files.clone().into_iter()
    }

    fn camera(&self) -> Option<crate::serializer::CameraData> {
        todo!()
    }
}
