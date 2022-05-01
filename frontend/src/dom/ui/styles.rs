use once_cell::sync::Lazy;
use dominator::{class, pseudo};
use crate::dom::theme;

pub const UI_CONTAINER:Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "flex")
        .style("flex-direction", "row")
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
    }
});

pub const HEADER:Lazy<String> = Lazy::new(|| {
    class! {
        .style("background-color", theme::GREEN)
        .style("color", "white")
        .style("padding", "10px")
    }
});

pub const BUTTON:Lazy<String> = Lazy::new(|| {
    class! {
        .style("background-color", theme::BLUE) 
        .style("color", theme::WHITE)
        .style("cursor", "pointer")
        .style("padding", "10px")
        .pseudo!(":hover", {
          .style("background-color", theme::BLUE_HOVER)
        })
    }
});
