use crate::{Engine, Float};
use sdl2::event::Event;

pub trait Game {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        event.into()
    }

    fn update(&mut self, dt: Float, engine: &Engine);
    fn render(&mut self, engine: &mut Engine);
}
