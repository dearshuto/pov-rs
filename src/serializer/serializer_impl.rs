use super::{detail, ISceneProvider};

pub struct Serializer;

impl Serializer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize<W, T>(&self, mut writer: W, scene_provider: &T) -> Result<(), ()>
    where
        W: std::io::Write,
        T: ISceneProvider,
    {
        // ヘッダーの出力
        for include in scene_provider.includes() {
            let str = format!("#include \"{}\"", include);

            let Ok(_) = writer.write_all(str.as_bytes()) else {
                return Err(());
            };

            continue;
        }

        // カメラの出力
        if let Some(camera_data) = scene_provider.camera() {
            detail::Camera::new()
                .serialize(writer, &camera_data)
                .unwrap();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::serializer::CameraData;

    use super::*;

    struct SceneProvider {}
    impl ISceneProvider for SceneProvider {
        fn includes(&self) -> impl Iterator<Item = String> {
            vec!["AAA.inc".to_string()].into_iter()
        }

        fn camera(&self) -> Option<CameraData> {
            None
        }
    }

    struct StringWriter(String);
    impl std::io::Write for &mut StringWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let s = std::str::from_utf8(buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            self.0.push_str(s);
            Ok(s.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn include() {
        let serializer = Serializer::new();

        let mut str = StringWriter(String::new());
        serializer.serialize(&mut str, &SceneProvider {}).unwrap();
        assert_eq!(str.0, "#include \"AAA.inc\"");
    }
}
