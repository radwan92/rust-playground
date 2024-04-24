use engine::{Engine, Float, Game};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

struct BasicGame {
    x: Float,
    y: Float,
    speed: Float,
}

impl Game for BasicGame {
    fn update(&mut self, dt: Float, eng: &Engine) {
        if eng.is_key_pressed(Keycode::Left) {
            self.x -= dt * self.speed;
        }
        if eng.is_key_pressed(Keycode::Right) {
            self.x += dt * self.speed;
        }
        if eng.is_key_pressed(Keycode::Up) {
            self.y -= dt * self.speed;
        }
        if eng.is_key_pressed(Keycode::Down) {
            self.y += dt * self.speed;
        }
    }

    fn render(&mut self, engine: &mut Engine) {
        engine.draw_rect(self.x as i32, self.y as i32, 20, 20, Color::RGB(0, 255, 0));
    }
}

fn main() {
    engine::create(
        BasicGame {
            x: 0.0,
            y: 0.0,
            speed: 250.0,
        },
        String::from("Basic Sample"),
    )
    .with_stretched_dimensions(1)
    .start();
}
