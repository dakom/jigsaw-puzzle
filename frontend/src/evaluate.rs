use shipyard::*;
use crate::media::MediaView;
use shipyard_scenegraph::prelude::*;

#[derive(Component)]
pub struct ToEvaluate; 

pub fn evaluate_sys(
    media: MediaView,
    to_evaluates: View<ToEvaluate>,
    translations: View<Translation>
) {
}
