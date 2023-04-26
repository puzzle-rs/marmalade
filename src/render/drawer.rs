use web_sys::HtmlImageElement;

use super::Color;

pub trait Drawer {
    fn draw_rect(&self, x: f64, y: f64, w: f64, h: f64, color: &Color);

    fn draw_image(&self, x: f64, y: f64, w: f64, h: f64, img: &HtmlImageElement);
}
