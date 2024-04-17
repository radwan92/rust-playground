use std::rc::Rc;
use std::cell::RefCell;
use crate::{Engine, Game};

pub struct EngineBuilder {
    game: Rc<RefCell<dyn Game>>,
    width: u32,
    height: u32,
}

pub fn create<T>(game: T) -> EngineBuilder
    where T: Game + 'static {
    EngineBuilder {
        game: Rc::new(RefCell::new(game)),
        width: 800,
        height: 600,
    }
}

impl EngineBuilder {
    pub fn with_dimensions(mut self, width: u32, height: u32) -> EngineBuilder {
        self.width = width;
        self.height = height;
        self
    }

    pub fn build(self) -> Engine {
        Engine::new(self.game, self.width, self.height)
    }

    pub fn start(self) {
        self.build().start();
    }
}
