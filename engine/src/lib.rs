#[cfg(target_family = "wasm")]
pub mod emscripten;

mod time;
mod game;
mod engine_builder;

pub use game::Game;
pub use engine_builder::*;

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
}

impl Engine {
    pub fn new(game: Rc<RefCell<dyn Game>>, width: u32, height: u32) -> Engine {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video.window("Hello world", width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl.event_pump().unwrap();

        Engine { running: true, event_pump, canvas, game, time: time::now() }
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
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        self.event_pump.keyboard_state().is_scancode_pressed(Scancode::from_keycode(keycode).unwrap())
    }

    fn create_main_loop(engine: Rc<RefCell<Self>>) -> impl FnMut() {
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

            engine.game.borrow_mut().update(dt.as_secs_f64() as Float, engine);

            let canvas = &mut engine.canvas;

            canvas.set_draw_color(Color::RGB(64, 64, 64));
            canvas.clear();

            engine.game.borrow_mut().render(canvas);

            canvas.present();
        }
    }
}
