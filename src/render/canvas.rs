use glam::DVec2;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement};

use super::{Color, Drawer};

pub struct Canvas {
    canvas: HtmlCanvasElement,
    gc: CanvasRenderingContext2d,
}

impl Canvas {
    #[must_use]
    pub fn new(document: &Document, canvas_id: &str) -> Self {
        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let gc = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self { canvas, gc }
    }

    pub fn clear(&self, clear_color: &Color) {
        let window = window().unwrap();

        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();

        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);

        self.gc.set_image_smoothing_enabled(false);

        self.draw_rect(DVec2::ZERO, DVec2::new(width, height), clear_color);
    }
}

impl Drawer for Canvas {
    fn draw_rect(&self, pos: DVec2, size: DVec2, color: &Color) {
        self.gc.set_fill_style(&color.to_css_color().into());

        self.gc.set_global_alpha(color.a as f64 / 255.);

        self.gc.fill_rect(pos.x, pos.y, size.x, size.y);
    }

    fn draw_image(&self, pos: DVec2, size: DVec2, img: &web_sys::HtmlImageElement) {
        self.gc.set_global_alpha(1.);

        self.gc
            .draw_image_with_html_image_element_and_dw_and_dh(img, pos.x, pos.y, size.x, size.y)
            .unwrap();
    }
}
