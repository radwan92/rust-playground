use std::rc::Rc;
use std::cell::RefCell;
use sdl2::pixels::Color;
use crate::{Dimensions, Engine, Game};

pub struct EngineBuilder {
    game: Rc<RefCell<dyn Game>>,
    dimensions: Dimensions,
    background_color: Color,
}

pub fn create<T>(game: T) -> EngineBuilder
    where T: Game + 'static {
    EngineBuilder {
        game: Rc::new(RefCell::new(game)),
        dimensions: Dimensions::default(),
        background_color: Color::BLACK,
    }
}

impl EngineBuilder {
    pub fn with_background_color(mut self, color: Color) -> EngineBuilder {
        self.background_color = color;
        self
    }

    pub fn with_dimensions(mut self, point_size: u32, width_in_points: u32, height_in_points: u32) -> EngineBuilder {
        self.dimensions = Dimensions::new(point_size, width_in_points, height_in_points);
        self
    }
    pub fn with_stretched_dimensions(mut self, point_size: u32) -> EngineBuilder {
        let width: u32;
        let height: u32;

        #[cfg(not(target_family = "wasm"))]
        {
            let sdl = sdl2::init().unwrap();
            let video = sdl.video().unwrap();
            let screen_size = video.display_bounds(0).unwrap();
            (width, height) = (screen_size.width(), screen_size.height());
        }

        #[cfg(target_family = "wasm")]
        {
            (width, height) = crate::emscripten::get_screen_size();
        }

        let (dim_width, dim_height) = (width / point_size, height / point_size);
        self.dimensions = Dimensions::new(point_size, dim_width, dim_height);

        self
    }

    pub fn build(self) -> Engine {
        Engine::new(self.game, self.dimensions)
    }

    pub fn start(self) {
        self.build().start();
    }
}
