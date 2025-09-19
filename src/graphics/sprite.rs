use crate::graphics::{Color, Sprite};
use anyhow::*;
use std::path::Path;

impl Sprite {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let img = image::ImageReader::open(path)?.decode()?.into_rgba8();

        let mut vec = Vec::new();
        for i in (0..img.len()).step_by(4) {
            let rgba: &[u8] = &img.as_raw()[i..i+4];
            vec.push(Color::from_u8(rgba));
        }

        Ok(Sprite{
            width: img.width(),
            height: img.height(),
            rgba: vec,
        })

    }
}