use image::{DynamicImage, GenericImageView};

use super::*;
use image::io::Reader as ImageReader;

pub struct ImageTexture {
    pub(crate) data: Option<DynamicImage>,
}

impl ImageTexture {
    pub fn from_img(path: &str) -> Self {
        Self {
            data: Some(
                ImageReader::open(path)
                    .expect("无效路径")
                    .decode()
                    .expect("读取图片失败"),
            ),
        }
    }
    pub fn new() -> Self {
        Self { data: None }
    }
    pub fn load_img(&mut self, path: &str) {
        self.data = Some(
            ImageReader::open(path)
                .expect("无效路径")
                .decode()
                .expect("读取图片失败"),
        );
    }
    pub fn clear_img(&mut self) {
        self.data.take();
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: (f32, f32), _p: Point3) -> Color {
        if let Some(img) = &self.data {
            // v是仰角，越大越靠上，但图像y越大是越下方，所以要用1减去它否则地球会倒置
            let uv = (uv.0.clamp(0., 1.), 1.0 - uv.1.clamp(0., 1.));
            let (mut i, mut j) = (
                (uv.0 * img.width() as f32) as u32,
                (uv.1 * img.height() as f32) as u32,
            );

            if i >= img.width() {
                i = img.width() - 1;
            }
            if j >= img.height() {
                j = img.height() - 1;
            }

            let color_scale = 1. / 255.;

            let pixel = img.get_pixel(i, j);

            Color::new(
                color_scale * pixel.0[0] as f32,
                color_scale * pixel.0[1] as f32,
                color_scale * pixel.0[2] as f32,
            )
        } else {
            Color::new(0., 1., 1.)
        }
    }
}
