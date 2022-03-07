use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::Path, collections::HashMap
};
use image::{GenericImageView};
use super::{info::{Info, ImageInfo}, config};


impl Info {
    pub fn read() -> Self {

        let mut images = _read_images("media/pieces.json", config::DEBUG_LIMIT).unwrap();

        // get completed puzzle size
        let (puzzle_width, puzzle_height) = images
            .iter()
            .fold((0,0), |(mut puzzle_width, mut puzzle_height), image| {
                let (image_width, image_height) = image.dimensions();
                let right = image.dest_x + image_width;
                //top-left origin, so we accumulate towards bottom
                let bottom = image.dest_y + image_height;

                if right > puzzle_width {
                    puzzle_width = right;
                }

                if bottom > puzzle_height {
                    puzzle_height = bottom;
                }

                (puzzle_width, puzzle_height)

            });

        // change 0,0 to be bottom left
        // or in other words, top y is puzzle_height (with piece anchored to its bottom-left)
        images
            .iter_mut()
            .for_each(|image| {
                image.dest_y = puzzle_height - (image.dest_y + image.height());
            });


        Self {
            images,
            puzzle_width,
            puzzle_height
        }
    }
}


fn _read_images<P: AsRef<Path>>(path: P, limit: Option<usize>) -> Result<Vec<ImageInfo>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let data: HashMap<String, (u32, u32, u32, u32)> = serde_json::from_reader(reader)?;
    let limit = limit.unwrap_or(data.len());

    let mut list:Vec<(String, (u32, u32, u32, u32))> = data.into_iter().collect();

    list.sort_by(|a, b| {
        let a_id:u32 = a.0.parse().unwrap();
        let b_id:u32 = b.0.parse().unwrap();
        a_id.cmp(&b_id)
    });

    Ok(list
        .into_iter()
        .take(limit)
        .map(|(id, (dest_x,dest_y,_,_))| {
            let img = image::open(format!("media/raster/{}.png", id)).unwrap();
            ImageInfo {
                id,
                img,
                dest_x,
                dest_y
            }
        })
        .collect()
    )
}
