use cgmath::Vector3;
use derive_new::new;
use procedural::*;

#[cfg(feature = "debug")]
use crate::graphics::{Camera, MarkerRenderer, Renderer};
#[cfg(feature = "debug")]
use crate::world::MarkerIdentifier;

#[derive(PrototypeElement, PrototypeWindow, new)]
#[window_title("Sound Source")]
pub struct SoundSource {
    pub name: String,
    pub sound_file: String,
    pub position: Vector3<f32>,
    pub volume: f32,
    pub width: usize,
    pub height: usize,
    pub range: f32,
    pub cycle: f32,
}

impl SoundSource {
    pub fn offset(&mut self, offset: Vector3<f32>) {
        self.position += offset;
    }

    #[cfg(feature = "debug")]
    pub fn render_marker<T>(
        &self,
        render_target: &mut T::Target,
        renderer: &T,
        camera: &dyn Camera,
        marker_identifier: MarkerIdentifier,
        hovered: bool,
    ) where
        T: Renderer + MarkerRenderer,
    {
        renderer.render_marker(render_target, camera, marker_identifier, self.position, hovered);
    }
}
