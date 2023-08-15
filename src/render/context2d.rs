use std::f64::consts::TAU;

use glam::{UVec2, Vec2};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, ImageBitmap, OffscreenCanvas, OffscreenCanvasRenderingContext2d};

use crate::global::window;

use super::Color;

pub struct Context2d {
    canvas: OffscreenCanvas,
    gc: OffscreenCanvasRenderingContext2d,
}

impl Context2d {
    #[must_use]
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        let canvas = canvas.transfer_control_to_offscreen().unwrap();

        let gc = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<OffscreenCanvasRenderingContext2d>()
            .unwrap();

        Self { canvas, gc }
    }

    #[must_use]
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

    pub fn clear(&self, clear_color: Color) {
        let window = window();

        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();

        self.gc.clear_rect(0., 0., width, height);

        self.draw_rect(
            Vec2::ZERO,
            Vec2::new(width as f32, height as f32),
            clear_color,
        );
    }

    pub fn set_size(&self, size: UVec2) {
        if size.x != self.canvas.width() || size.y != self.canvas.height() {
            self.canvas.set_width(size.x);
            self.canvas.set_height(size.y);
            //self.gc.set_image_smoothing_enabled(false);
        }
    }

    pub fn fit_screen(&self) {
        let window = window();

        self.set_size(UVec2::new(
            window.inner_width().unwrap().as_f64().unwrap() as u32,
            window.inner_height().unwrap().as_f64().unwrap() as u32,
        ));
    }

    fn set_color(&self, color: Color) {
        self.gc.set_fill_style(&color.to_css_color().into());

        self.gc.set_global_alpha(color.a as f64 / 255.);
    }

    pub fn draw_rect(&self, pos: Vec2, size: Vec2, color: Color) {
        self.set_color(color);
        self.gc
            .fill_rect(pos.x as f64, pos.y as f64, size.x as f64, size.y as f64);
    }

    pub fn draw_disk(&self, center: Vec2, radius: f32, color: Color) {
        self.set_color(color);

        self.gc.begin_path();

        self.gc
            .arc(center.x as f64, center.y as f64, radius as f64, 0., TAU)
            .unwrap();

        self.gc.fill();
    }

    pub fn draw_image(&self, pos: Vec2, size: Vec2, img: &ImageBitmap) {
        self.gc.set_global_alpha(1.);

        self.gc
            .draw_image_with_image_bitmap_and_dw_and_dh(
                img,
                pos.x as f64,
                pos.y as f64,
                size.x as f64,
                size.y as f64,
            )
            .unwrap();
    }

    pub fn draw_text(&self, text: &str, pos: Vec2, height: f32, color: Color) {
        self.set_color(color);

        self.gc.set_font(&format!("{height}px sans-serif"));

        self.gc.fill_text(text, pos.x as f64, pos.y as f64).unwrap();
    }
}
