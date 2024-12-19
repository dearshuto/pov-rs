#[derive(Debug, Default)]
struct Scene {
    includes: Vec<String>,
}
impl pov_rs::ISceneBuilder for Scene {
    fn add_include_path<T>(&mut self, path: T)
    where
        T: AsRef<std::path::Path>,
    {
        self.includes
            .push(path.as_ref().to_str().unwrap().to_string());
    }

    fn set_camera(&mut self, _camera_data: pov_rs::CameraData) {
        todo!()
    }
}

fn main() {
    let pov_source_bytes = include_str!("simple_deserialize.pov").as_bytes();

    let mut reader = Scene::default();
    pov_rs::Deserializer::new()
        .deserialize(pov_source_bytes, &mut reader)
        .unwrap();

    for include in reader.includes {
        println!("{}", include);
    }
}
