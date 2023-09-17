use fontdue::Font;

#[must_use]
pub fn from_bytes(bytes: &[u8]) -> Font {
    fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default()).expect("Couldn't parse font")
}
