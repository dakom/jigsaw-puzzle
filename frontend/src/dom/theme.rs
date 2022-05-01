use std::fmt;
use dominator::stylesheet;
use cfg_if::cfg_if;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static THEME:Lazy<Theme> = Lazy::new(|| {
    Theme::new()
}); 

pub type Color = &'static str;
pub struct ThemeButtonColor {
    pub bg_regular: &'static str,
    pub bg_hover: &'static str,
    pub text_regular: &'static str,
    pub text_hover: &'static str
}

pub struct Theme {
    pub nav_bg_color: &'static str,
    pub button_color_blue: ThemeButtonColor,
    pub button_color_red: ThemeButtonColor,
    pub h1_color: &'static str,
    pub error_color: &'static str,
    pub input_text_color: &'static str,
    pub input_border_color: &'static str,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            nav_bg_color: BG_COLOR_1,
            button_color_blue: BUTTON_COLOR_1, 
            button_color_red: BUTTON_COLOR_2, 
            h1_color: TEXT_COLOR_1, 
            error_color: TEXT_COLOR_2, 
            input_text_color: TEXT_COLOR_1, 
            input_border_color: BORDER_COLOR_1, 
        }
    }
}

const TEXT_COLOR_1:Color = "#2a2a2a";
const TEXT_COLOR_2:Color = "tomato";

const BG_COLOR_1:Color = "beige";
const BORDER_COLOR_1:Color = "darkgrey";
const BUTTON_COLOR_1:ThemeButtonColor = ThemeButtonColor { 
    bg_regular: "cornflowerblue", 
    bg_hover: "dodgerblue",
    text_regular: "white", 
    text_hover: "white",
};

const BUTTON_COLOR_2:ThemeButtonColor = ThemeButtonColor { 
    bg_regular: "tomato", 
    bg_hover: "crimson",
    text_regular: "white", 
    text_hover: "white",
};

pub fn init_stylesheet() {
    stylesheet! {
        ":root", {
            .style("font-family", "Arial, Helvetica, sans-serif")
            .style("box-sizing", "border-box")
            //let's us use rem units but have it by default equal px
            .style("font-size", "1.6px")
        }
    }
    stylesheet! {
        "*, *:before, *:after", {
            .style("box-sizing", "inherit")
        }
    }

    stylesheet! {
        "html, body", {
            .style("margin", "0")
            .style("padding", "0")
        }
    }

    stylesheet! {
        "body", {
            .style("font-size", "16rem")
        }
    }

    stylesheet! {
        "a", {
            .style("text-decoration", "none")
        }
    }

    stylesheet! {
        "ul", {
            .style("list-style", "none")
        }
    }

    stylesheet! {
        "h1", {
            .style("font-size", "20rem")
            .style("color", THEME.h1_color)
        }
    }

    stylesheet! {
        "h2", {
            .style("font-size", "15rem")
        }
    }

    stylesheet! {
        "h3", {
            .style("font-size", "11.7rem")
        }
    }
    stylesheet! {
        "h4", {
            .style("font-size", "10rem")
        }
    }
    stylesheet! {
        "h5", {
            .style("font-size", "8.3rem")
        }
    }
    stylesheet! {
        "h6", {
            .style("font-size", "6.7rem")
        }
    }
}
