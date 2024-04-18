use std::rc::Rc;
use std::cell::RefCell;
use crate::{Dimensions, Engine, Game};

pub struct EngineBuilder {
    game: Rc<RefCell<dyn Game>>,
    dimensions: Dimensions,
}

pub fn create<T>(game: T) -> EngineBuilder
    where T: Game + 'static {
    EngineBuilder {
        game: Rc::new(RefCell::new(game)),
        dimensions: Dimensions::default(),
    }
}

impl EngineBuilder {
    pub fn with_dimensions(mut self, point_size: u32, width: u32, height: u32) -> EngineBuilder {
        self.dimensions = Dimensions::new(point_size, width, height);
        self
    }

    pub fn build(self) -> Engine {
        Engine::new(self.game, self.dimensions)
    }

    pub fn start(self) {
        self.build().start();
    }
}
