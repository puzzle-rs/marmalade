use glam::Vec2;
use marmalade::dom_stack;
use marmalade::draw_scheduler;
use marmalade::image;
use marmalade::input::{keyboard, Key};
use marmalade::render::object2d::Circle2D;
use marmalade::render::Color;
use marmalade::render::Context2d;
use marmalade::render::Webgl2d;

async fn async_main() {
    dom_stack::set_title("Hello World");

    // Create an HtmlCanvas where the game will be displayed
    let main_canvas = dom_stack::create_full_screen_canvas();
    // Add the Html canvas to the dom
    dom_stack::stack_node(&main_canvas);

    // Create an other canvas for drawing text
    let text_canvas = dom_stack::create_full_screen_canvas();
    // Add this canvas on top of the other
    dom_stack::stack_node(&text_canvas);

    // Create a context for drawing the "game", webgl context is fast and flexible, but a bit more complicated and can't draw text
    let mut wgl2d = Webgl2d::new(&main_canvas);

    // Create a simple 2d context for drawing text
    let ctx2d = Context2d::new(&text_canvas);

    // Load an image
    let image = image::from_bytes(include_bytes!("../../../resources/logo.png")).await;

    // Upload the image to the GPU
    let texture = wgl2d.create_texture(&image);

    let mut position = Vec2::new(200., 200.);

    // Closure called for every frame
    draw_scheduler::set_on_draw(move || {
        // Move the sprite with keyboard
        if keyboard::is_down(Key::A) {
            position.x -= 4.;
        }
        if keyboard::is_down(Key::D) {
            position.x += 4.;
        }
        if keyboard::is_down(Key::S) {
            position.y -= 4.;
        }
        if keyboard::is_down(Key::W) {
            position.y += 4.;
        }

        // Set size of the canvas to the same as screen
        wgl2d.fit_screen();
        ctx2d.fit_screen();

        // Set the view matrix so that coordinates corresponds to pixels on the canvas
        wgl2d.pixel_perfect_view();

        // Clear canvas to black
        wgl2d.clear(Color::rgb(0, 0, 0));
        ctx2d.clear(Color::rgba(0, 0, 0, 0)); // Fully transparent

        // Create an hexagon (circle with six sides) with our texture and draw it
        let textured_circle = Circle2D::new_textured(position, 100., 6, texture.clone());
        wgl2d.draw(&textured_circle);

        // Draw a transparent red hexagon on top of it
        let colored_circle = Circle2D::new_colored(position, 100., 6, Color::rgba(127, 0, 0, 127));
        wgl2d.draw(&colored_circle);

        ctx2d.draw_text(
            "Move with W A S D",
            Vec2::new(50., 80.),
            50.,
            Color::rgb(255, 255, 255),
        );
    });
}

fn main() {
    // Redirect rust panics to the console for easier debugging
    console_error_panic_hook::set_once();

    // Start the async_main function, some marmalade functionalities require an async context
    wasm_bindgen_futures::spawn_local(async_main());
}
