use glam::DVec2;
use marmalade::draw_scheduler::DrawScheduler;
use marmalade::image;
use marmalade::input::{Key, Keyboard};
use marmalade::render::{Canvas, Color};

async fn async_main() {
    // Load an image for later drawing it
    let image = image::from_bytes(
        include_bytes!("../../../resources/logo.png"),
        &image::Format::Png,
    )
    .await
    .unwrap();

    let mut position = DVec2::new(0., 0.);
    let size = DVec2::new(100., 100.);

    // Create a keyboard for reading user inputs
    let keyboard = Keyboard::new();

    // Create a canvas for drawing the "game"
    let canvas = Canvas::new("canvas");

    // Create a scheduler for calling a Closure on every new frame
    let draw_scheduler = DrawScheduler::new();

    // Closure called for every frame
    draw_scheduler.set_on_draw(move || {
        // Move the sprite with keyboard
        if keyboard.is_down(&Key::A) {
            position.x -= 4.;
        }
        if keyboard.is_down(&Key::D) {
            position.x += 4.;
        }
        if keyboard.is_down(&Key::W) {
            position.y -= 4.;
        }
        if keyboard.is_down(&Key::S) {
            position.y += 4.;
        }

        // Clear canvas to black
        canvas.clear(&Color::rgb(0, 0, 0));

        // Draw the 100px by 100px sprite at coordinates x, y
        canvas.draw_image(position, size, &image);

        // Draw a transparent rectangle on top of the sprite to give it a red tint
        canvas.draw_rect(position, size, &Color::rgba(127, 0, 0, 127));
    });
}

fn main() {
    // Redirect rust panics to the console for easier debugging
    console_error_panic_hook::set_once();

    // Start the async_main function, some marmalade functionalities require an async context
    wasm_bindgen_futures::spawn_local(async_main());
}
