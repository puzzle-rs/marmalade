use marmalade::glam::DVec2;
use marmalade::input::Key;
use marmalade::marmalade_context::MarmaladeContext;
use marmalade::render::{Color, Drawer};
use marmalade::wasm_bindgen_futures::spawn_local;
use marmalade::{console_error_panic_hook, image};
use std::cell::RefCell;
use std::rc::Rc;

async fn async_main() {
    // Create a marmalade context, this will then be used for operations like drawing or reading keyboard
    let mc = Rc::new(RefCell::new(MarmaladeContext::new("canvas")));

    // Load an image to use it as a sprite
    let image = image::from_bytes(
        include_bytes!("../../../resources/logo.png"),
        &image::Format::Png,
    )
    .await
    .unwrap();

    // Create the variables that will be sent to the following closure
    let mc_clone = mc.clone();
    let mut position = DVec2::new(0., 0.);
    let size = DVec2::new(100., 100.);

    // set_on_draw is used to define a closure that will be called for every frame
    mc.borrow_mut().set_on_draw(move || {
        // Retrieve the MarmaladeContext for local usage
        let mc = mc_clone.borrow();

        // Move the sprite with keyboard
        if mc.keyboard.is_down(&Key::A) {
            position.x -= 4.;
        }
        if mc.keyboard.is_down(&Key::D) {
            position.x += 4.;
        }
        if mc.keyboard.is_down(&Key::W) {
            position.y -= 4.;
        }
        if mc.keyboard.is_down(&Key::S) {
            position.y += 4.;
        }

        // Clear canvas to black
        mc.canvas.clear(&Color::rgb(0, 0, 0));

        // Draw the 100px by 100px sprite at coordinates x, y
        mc.canvas.draw_image(position, size, &image);

        // Draw a transparent rectangle on top of the sprite to give it a red tint
        mc.canvas
            .draw_rect(position, size, &Color::rgba(127, 0, 0, 127));
    });
}

fn main() {
    // Redirect rust panics to the console for easier debugging
    console_error_panic_hook::set_once();

    // Start the async_main function, some marmalade functionalities require an async context
    spawn_local(async_main());
}
