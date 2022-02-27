pub const ZOOM_AMOUNT:f64 = 0.1;
pub const ZOOM_MIN:f64 = 0.1;
pub const ZOOM_MAX:f64 = 3.0;

const PUZZLE_NAME:&str = "landscape";

pub fn get_media_href(path:&str) -> String {
    format!("media/{}", path)
}
pub fn get_puzzle_href(path:&str) -> String {
    get_media_href(&format!("puzzle/{}/{}", PUZZLE_NAME, path))
}
