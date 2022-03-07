use std::ops::Deref;
use image::{GenericImageView, DynamicImage};

pub struct Info {
    pub images: Vec<ImageInfo>,
    pub puzzle_width: u32,
    pub puzzle_height: u32,
}

pub struct ImageInfo {
    pub id: String,
    pub dest_x: u32,
    pub dest_y: u32,
    pub(super) img: DynamicImage
}

impl ImageInfo {
    pub fn _width(&self) -> u32 {
        self.dimensions().0
    }
    pub fn height(&self) -> u32 {
        self.dimensions().1
    }
}

impl Deref for ImageInfo {
    type Target = DynamicImage;

    fn deref(&self) -> &DynamicImage {
        &self.img
    }
}


