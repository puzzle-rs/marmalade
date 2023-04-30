use glam::DVec2;
use web_sys::HtmlImageElement;

use super::Color;

pub trait Drawer {
    fn draw_rect(&self, pos: DVec2, size: DVec2, color: &Color);

    fn draw_image(&self, pos: DVec2, size: DVec2, img: &HtmlImageElement);
}
