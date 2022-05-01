use super::state::*;

impl ButtonState {
    pub fn as_str(&self) -> &'static str{
        match self {
            Self::Reset => "reset",
            Self::Start => "start",
        }
    }
}

impl HeaderState {
    pub fn as_str(&self) -> &'static str{
        match self {
            Self::Loading => "Loading...",
            Self::Prepping => "Preparing...",
            Self::Connecting => "Connecting...",
            Self::Playing => "click and drag, space-click to pan, mouse wheel to zoom"
        }
    }
}

