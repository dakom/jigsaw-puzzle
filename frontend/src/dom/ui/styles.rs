use once_cell::sync::Lazy;
use dominator::class;
use crate::dom::theme::THEME;

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
        .style("background-color", "forestgreen")
        .style("color", "white")
        .style("padding", "10px")
    }
});

pub const BUTTON:Lazy<String> = Lazy::new(|| {
    class! {
        .style("background-color", "dodgerblue")
        .style("color", "white")
        .style("cursor", "pointer")
        .style("padding", "10px")
    }
});
