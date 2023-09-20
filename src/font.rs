use std::rc::Rc;

use fontdue::Font;

thread_local! {
    static MONOGRAM: Rc<Font> = Rc::new(from_bytes(include_bytes!("../resources/fonts/monogram-extended.ttf"), 16.));
}

#[must_use]
pub fn monogram() -> Rc<Font> {
    MONOGRAM.with(Clone::clone)
}

#[must_use]
pub fn from_bytes(bytes: &[u8], px: f32) -> Font {
    

    let settings = fontdue::FontSettings {
        scale: px,
        ..Default::default()
    };

    fontdue::Font::from_bytes(bytes, settings).expect("Couldn't parse font")
}
