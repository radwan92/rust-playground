use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use crate::{Engine, Float};

pub trait Game {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        event.into()
    }

    fn update(&mut self, dt: Float, eng: &Engine);
    fn render(&self, canvas: &mut WindowCanvas);
}
