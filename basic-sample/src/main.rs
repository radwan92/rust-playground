use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use engine::{Engine, Float, Game};

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

    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(sdl2::rect::Rect::new(self.x as i32, self.y as i32, 20, 20)).unwrap();
    }
}

fn main() {
    engine::create(BasicGame { x: 0.0, y: 0.0, speed: 250.0 })
        .start();
}
