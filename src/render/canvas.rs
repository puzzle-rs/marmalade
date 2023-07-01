use glam::{DVec2, UVec2};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, OffscreenCanvas, OffscreenCanvasRenderingContext2d};

use super::Color;

pub struct Canvas {
    canvas: OffscreenCanvas,
    gc: OffscreenCanvasRenderingContext2d,
}

impl Canvas {
    #[must_use]
    pub fn new(canvas_id: &str) -> Self {
        let canvas = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap()
            .transfer_control_to_offscreen()
            .unwrap();

        let gc = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<OffscreenCanvasRenderingContext2d>()
            .unwrap();

        Self { canvas, gc }
    }

    pub fn new_offscreen(size: UVec2) -> Self {
        let canvas = OffscreenCanvas::new(size.x, size.y).unwrap();

        let gc = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<OffscreenCanvasRenderingContext2d>()
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

    pub fn set_size(&self, size: UVec2) {
        self.canvas.set_width(size.x);
        self.canvas.set_height(size.y);
    }

    pub fn fit_screen(&self) {
        let window = window().unwrap();

        self.set_size(UVec2::new(
            window.inner_width().unwrap().as_f64().unwrap() as u32,
            window.inner_height().unwrap().as_f64().unwrap() as u32,
        ));
    }

    pub fn draw_rect(&self, pos: DVec2, size: DVec2, color: &Color) {
        self.gc.set_fill_style(&color.to_css_color().into());

        self.gc.set_global_alpha(color.a as f64 / 255.);

        self.gc.fill_rect(pos.x, pos.y, size.x, size.y);
    }

    pub fn draw_image(&self, pos: DVec2, size: DVec2, img: &web_sys::HtmlImageElement) {
        self.gc.set_global_alpha(1.);

        self.gc
            .draw_image_with_html_image_element_and_dw_and_dh(img, pos.x, pos.y, size.x, size.y)
            .unwrap();
    }
}
