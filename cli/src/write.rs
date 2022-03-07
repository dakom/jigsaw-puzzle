use super::{info::Info, config};
use image::{GenericImage, RgbaImage, GenericImageView};
use std::ops::Deref;
use std::fs::File;
use serde::Serialize;

impl Info {
    pub fn write(&self) {
        let puzzle:Puzzle = self.into();

        let file = File::create(format!("{}/{}.json", config::OUTPUT_DIR, config::OUTPUT_FILENAME)).unwrap();
        serde_json::to_writer_pretty(file, &puzzle).unwrap();


        let mut imgbuf = RgbaImage::new(puzzle.atlas_width, puzzle.atlas_height);

        for (index, info_image) in self.images.iter().enumerate() {
            let puzzle_image = &puzzle.images[index];
           
            let (src_x, src_y, dest_x, dest_y, width, height) = *puzzle_image;

            println!("{}: src_x: {}, src_y: {}, dest_x: {}, dest_y: {}, height: {}, width: {}",
                index, src_x, src_y, dest_x, dest_y, height, width
            );
            imgbuf.copy_from(info_image.deref(), src_x, src_y).unwrap();
        }

        imgbuf.save(format!("{}/{}.png", config::OUTPUT_DIR, config::OUTPUT_FILENAME)).unwrap();
    }
}

#[derive(Serialize)]
pub(super) struct Puzzle {
    pub puzzle_width: u32,
    pub puzzle_height: u32,
    pub atlas_width: u32,
    pub atlas_height: u32,
    //src x,y, dest x,y and bitmap w,h
    pub images: Vec<(u32, u32, u32, u32, u32, u32)>
}

impl From<&Info> for Puzzle {
    fn from(info:&Info) -> Self {
        let mut images = Vec::new();


        let mut atlas_width = 0;
        let mut atlas_height = 0;
        let mut x = 0;
        let mut y = config::PADDING_PX;
        let mut largest_y = 0;

        for image in info.images.iter() {
            let (image_width, image_height) = image.dimensions();
            x += config::PADDING_PX;

            //limiting right to puzzle_width isn't perfect, but the goal
            //is just to stay within reasonable limits, not to make a perfect square
            if (x + image_width + config::PADDING_PX) >= info.puzzle_width {
                x = config::PADDING_PX;
                y += largest_y + config::PADDING_PX;
                largest_y = 0;
            }

            images.push((x, y, image.dest_x, image.dest_y, image_width, image_height));

            x += image_width;

            let right = x + config::PADDING_PX;
            let bottom = y + image_height + config::PADDING_PX;

            if right > atlas_width {
                atlas_width = right;
            }

            if bottom > atlas_height {
                atlas_height = bottom;
            }

            if image_height > largest_y {
                largest_y = image_height;
            }

        }

        Self {
            puzzle_width: info.puzzle_width,
            puzzle_height: info.puzzle_height,
            atlas_width,
            atlas_height,
            images
        }
    }
}
