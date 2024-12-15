struct SimpleScene;
impl pov_rs::ISceneProvider for SimpleScene {
    fn includes(&self) -> impl Iterator<Item = String> {
        [].into_iter()
    }

    fn camera(&self) -> Option<pov_rs::CameraData> {
        Some(pov_rs::CameraData {
            location: [5.0, 5.0, -10.0],
            look_at: [0.0, 0.0, 0.0],
            angle: 20.0,
        })
    }
}

fn main() {
    let file = std::fs::File::create("simple.pov").unwrap();
    pov_rs::Serializer::new()
        .serialize(file, &SimpleScene {})
        .unwrap();
}
