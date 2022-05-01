use std::fmt;
use dominator::stylesheet;
use cfg_if::cfg_if;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub const BLUE:&'static str = "dodgerblue";
pub const BLUE_HOVER:&'static str = "rgb(20, 138, 255)";
pub const WHITE:&'static str = "white";
pub const RED:&'static str = "tomato";
pub const GREEN:&'static str = "forestgreen";


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
            .style("font-size", "12rem")
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
