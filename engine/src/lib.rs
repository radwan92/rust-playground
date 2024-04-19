#![deny(warnings)]

#[cfg(target_family = "wasm")]
pub mod emscripten;

mod time;
mod game;
mod engine_builder;
mod dimensions;
mod point;

pub use game::Game;
pub use engine_builder::create;
pub use dimensions::Dimensions;
pub use point::Point;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub type Float = f64;

pub struct Engine {
    running: bool,
    event_pump: sdl2::EventPump,
    canvas: WindowCanvas,
    game: Rc<RefCell<dyn Game>>,
    time: Duration,
    dimensions: Dimensions,
    background_color: Color,
}

// API
impl Engine {
    pub fn draw_point(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);

        let draw_rect = dimensions::point_at(&self.dimensions, x, y);
        self.canvas.fill_rect(draw_rect).unwrap()
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        self.canvas.set_draw_color(color);

        let draw_rect = dimensions::rect_at(&self.dimensions, x, y, width, height);
        self.canvas.fill_rect(draw_rect).unwrap()
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        self.event_pump.keyboard_state().is_scancode_pressed(Scancode::from_keycode(keycode).unwrap())
    }
}

// Initialization and main loop
impl Engine {
    pub fn new(
        game: Rc<RefCell<dyn Game>>,
        game_title: String,
        dimensions: Dimensions,
        background_color: Color) -> Engine
    {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video.window(game_title.as_str(), dimensions.pixel_width(), dimensions.pixel_height())
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl.event_pump().unwrap();

        Engine {
            running: true,
            event_pump,
            canvas,
            game,
            time: time::now(),
            dimensions,
            background_color,
        }
    }

    pub fn start(self) {
        let engine = Rc::new(RefCell::new(self));

        #[cfg(target_family = "wasm")]
        Engine::start_emscripten(engine);

        #[cfg(not(target_family = "wasm"))]
        Engine::start_desktop(engine);
    }

    #[cfg(target_family = "wasm")]
    fn start_emscripten(engine: Rc<RefCell<Engine>>) {
        emscripten::set_main_loop_callback(Engine::create_main_loop(engine));
    }

    #[cfg(not(target_family = "wasm"))]
    fn start_desktop(engine: Rc<RefCell<Engine>>) {
        let mut loop_func = Engine::create_main_loop(engine.clone());

        while engine.borrow().running {
            loop_func();
            std::thread::sleep(Duration::from_millis(16));
        }
    }

    fn create_main_loop(engine: Rc<RefCell<Self>>) -> impl FnMut() {
        let game = engine.borrow().game.clone();

        move || {
            let engine = &mut *engine.borrow_mut();
            let event_pump = &mut engine.event_pump;

            for event in event_pump.poll_iter() {
                if let Some(event) = engine.game.borrow_mut().handle_event(event) {
                    match event {
                        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            engine.running = false;
                        }
                        _ => ()
                    }
                }
            }

            let now = time::now();
            let dt = now - engine.time;
            engine.time = now;

            game.borrow_mut().update(dt.as_secs_f64() as Float, engine);

            engine.canvas.set_draw_color(engine.background_color);
            engine.canvas.clear();

            game.borrow_mut().render(engine);

            engine.canvas.present();
        }
    }
}
