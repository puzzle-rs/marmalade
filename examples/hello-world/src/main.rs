use glam::Vec2;
use marmalade::dom_stack;
use marmalade::draw_scheduler;
use marmalade::image;
use marmalade::input::{keyboard, Key};
use marmalade::render::{Canvas, Color};

async fn async_main() {
    dom_stack::set_title("Hello World");

    // Load an image for later drawing it
    let image = image::from_bytes(
        include_bytes!("../../../resources/logo.png"),
        &image::Format::Png,
    )
    .await
    .unwrap();

    // Create an HtmlCanvas where the game will be displayed
    let html_canvas = dom_stack::create_full_screen_canvas();

    // Add the Html canvas to the dom
    dom_stack::stack_node(&html_canvas);

    // Create a canvas for drawing the "game"
    let canvas = Canvas::new(&html_canvas);

    let mut position = Vec2::new(0., 0.);
    let size = Vec2::new(100., 100.);

    // Closure called for every frame
    draw_scheduler::set_on_draw(move || {
        // Move the sprite with keyboard
        if keyboard::is_down(Key::A) {
            position.x -= 4.;
        }
        if keyboard::is_down(Key::D) {
            position.x += 4.;
        }
        if keyboard::is_down(Key::W) {
            position.y -= 4.;
        }
        if keyboard::is_down(Key::S) {
            position.y += 4.;
        }

        // Set size of the canvas to the same as screen
        canvas.fit_screen();

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
