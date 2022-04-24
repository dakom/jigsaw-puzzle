pub const ZOOM_AMOUNT:f64 = 0.1;
pub const ZOOM_MIN:f64 = 0.1;
pub const ZOOM_MAX:f64 = 3.0;

const PUZZLE_NAME:&str = "landscape";

pub fn get_media_href(path:&str) -> String {
    if cfg!(feature = "remote-media") {
        format!("https://storage.googleapis.com/dakom-jigsaw-puzzle/media/{}", path)
    } else {
        format!("media/{}", path)
    }
}

pub fn get_puzzle_href(path:&str) -> String {
    get_media_href(&format!("puzzle/{}/{}", PUZZLE_NAME, path))
}

pub fn websocket_url() -> &'static str {
    if cfg!(feature = "dev") {
        "ws://127.0.0.1:8787/socket"
    } else {
        "wss://jigsaw-puzzle-worker.dakom.workers.dev/socket"
    }
}
