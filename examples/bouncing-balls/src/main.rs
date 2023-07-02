use std::cell::RefCell;
use std::time::Duration;

use glam::Vec2;
use marmalade::draw_scheduler::DrawScheduler;
use marmalade::global::window;
use marmalade::input::{Key, Keyboard};
use marmalade::render::{Canvas, Color};
use marmalade::tick_scheduler::TickScheduler;

const GRAVITY: Vec2 = Vec2::new(0., 0.0015);
const COLLISION_SMOOTHNESS: f32 = 0.003;

const FRICTION: f32 = 0.999;

struct Ball {
    position: Vec2,
    speed: Vec2,
    radius: f32,
}

impl Ball {
    pub fn new(position: Vec2, speed: Vec2, radius: f32) -> Self {
        Self {
            position,
            speed,
            radius,
        }
    }

    pub fn tick(&mut self) {
        self.speed += GRAVITY;
        self.position += self.speed;

        self.speed *= FRICTION;

        let window = window();

        let width = window.inner_width().unwrap().as_f64().unwrap() as f32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as f32;

        if self.position.x - self.radius < 0. {
            self.position.x = self.radius;
            self.speed.x = -self.speed.x;
        } else if self.position.x + self.radius > width {
            self.position.x = width - self.radius;
            self.speed.x = -self.speed.x;
        }

        if self.position.y - self.radius < 0. {
            self.position.y = self.radius;
            self.speed.y = -self.speed.y;
        } else if self.position.y + self.radius > height {
            self.position.y = height - self.radius;
            self.speed.y = -self.speed.y;
        }
    }

    pub fn collide(a: &mut Ball, b: &mut Ball) {
        let dist = a.position - b.position;

        let overlap = a.radius + b.radius - dist.length();

        if overlap > 0. {
            let push = dist.normalize_or_zero() * overlap * COLLISION_SMOOTHNESS;
            a.speed += push;
            b.speed -= push;
        }
    }
}

async fn async_main() {
    let mut balls = Vec::new();

    // Create a keyboard for reading user inputs
    let keyboard = Keyboard::new();

    // Create a canvas for drawing the "game"
    let canvas = Canvas::new("canvas");

    let mut tick_scheduler = TickScheduler::new(Duration::from_millis(1));

    // Create a scheduler for calling a Closure on every new frame
    let draw_scheduler = DrawScheduler::new();

    let mut write_instructions = true;

    // Closure called for every frame
    draw_scheduler.set_on_draw(move || {
        if keyboard.is_pressed(&Key::Space) {
            balls.push(RefCell::new(Ball::new(
                Vec2::new(30., 30.),
                Vec2::new(1., 0.),
                25.,
            )));

            write_instructions = false;
        }

        for _ in 0..tick_scheduler.tick_count() {
            for ball in &mut balls {
                ball.borrow_mut().tick();
            }

            for a in 0..balls.len() {
                for b in (a + 1)..balls.len() {
                    Ball::collide(&mut balls[a].borrow_mut(), &mut balls[b].borrow_mut());
                }
            }
        }

        // Set size of the canvas to the same as screen
        canvas.fit_screen();

        // Clear canvas to black, but with a transparent color to have a trail effect
        canvas.clear(&Color::rgba(0, 0, 0, 63));

        for ball in &balls {
            let ball = ball.borrow_mut();
            canvas.draw_disk(ball.position, ball.radius, &Color::rgb(255, 127, 0));
        }

        if write_instructions {
            canvas.draw_text(
                "Press SPACE to throw a ball",
                Vec2::new(50., 100.),
                50.,
                &Color::rgb(255, 255, 255),
            );
        }
    });
}

fn main() {
    // Redirect rust panics to the console for easier debugging
    console_error_panic_hook::set_once();

    // Start the async_main function, some marmalade functionalities require an async context
    wasm_bindgen_futures::spawn_local(async_main());
}
