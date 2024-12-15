use std::io::Write;

use crate::serializer::CameraData;

pub struct Camera;

impl Camera {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize<W>(&self, writer: W, camera_data: &CameraData) -> Result<(), ()>
    where
        W: std::io::Write,
    {
        let mut buf_writer = std::io::BufWriter::new(writer);

        buf_writer
            .write_all(format!("camera {{\n").as_bytes())
            .unwrap();
        buf_writer
            .write_all(
                format!(
                    "    location <{}, {}, {}>\n",
                    camera_data.location[0], camera_data.location[1], camera_data.location[2]
                )
                .as_bytes(),
            )
            .unwrap();
        buf_writer
            .write_all(
                format!(
                    "    look_at <{}, {}, {}>\n",
                    camera_data.look_at[0], camera_data.look_at[1], camera_data.look_at[2]
                )
                .as_bytes(),
            )
            .unwrap();
        buf_writer
            .write_all(format!("    angle {}\n", camera_data.angle).as_bytes())
            .unwrap();
        buf_writer.write_all(format!("}}\n").as_bytes()).unwrap();

        Ok(())
    }
}
