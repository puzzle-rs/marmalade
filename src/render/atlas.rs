use glam::Vec2;
use wasm_bindgen::JsCast;
use web_sys::{ImageBitmap, OffscreenCanvas, OffscreenCanvasRenderingContext2d};

const TEXTURE_SIZE: u32 = 4096;

pub const NO_TEXTURE_RECT: TextureRect = TextureRect {
    position: Vec2::ZERO,
    size: Vec2::ZERO,
};

pub struct TextureRect {
    pub position: Vec2,
    pub size: Vec2,
}

pub struct AtlasBuilder {
    x: u32,
    y: u32,
    next_y: u32,
    canvas: OffscreenCanvas,
    gc: OffscreenCanvasRenderingContext2d,
}

impl AtlasBuilder {
    pub fn new() -> AtlasBuilder {
        let canvas = OffscreenCanvas::new(TEXTURE_SIZE, TEXTURE_SIZE)
            .expect("Error, can't create canvas for texture atlas");

        let gc = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<OffscreenCanvasRenderingContext2d>()
            .unwrap();

        gc.set_fill_style(&"white".into());

        gc.fill_rect(0., 0., 1., 1.);

        AtlasBuilder {
            x: 1,
            y: 0,
            next_y: 1,
            canvas,
            gc,
        }
    }

    pub fn insert_image(&mut self, image: &ImageBitmap) -> TextureRect {
        if self.x + image.width() + 2 > TEXTURE_SIZE {
            self.x = 0;
            self.y = self.next_y;
        }

        assert!(
            self.x + image.width() + 2 <= TEXTURE_SIZE
                && self.y + image.height() + 2 <= TEXTURE_SIZE
        );

        let rect = TextureRect {
            position: Vec2::new(
                (self.x + 1) as f32 / TEXTURE_SIZE as f32,
                (self.y + 1) as f32 / TEXTURE_SIZE as f32,
            ),
            size: Vec2::new(
                image.width() as f32 / TEXTURE_SIZE as f32,
                image.height() as f32 / TEXTURE_SIZE as f32,
            ),
        };

        self.gc.set_image_smoothing_enabled(false);

        // Corners
        self.gc
            .draw_image_with_image_bitmap(image, self.x as f64, self.y as f64)
            .unwrap();
        self.gc
            .draw_image_with_image_bitmap(image, (self.x + 2) as f64, self.y as f64)
            .unwrap();
        self.gc
            .draw_image_with_image_bitmap(image, self.x as f64, (self.y + 2) as f64)
            .unwrap();
        self.gc
            .draw_image_with_image_bitmap(image, (self.x + 2) as f64, (self.y + 2) as f64)
            .unwrap();

        // Sides
        self.gc
            .draw_image_with_image_bitmap(image, self.x as f64, (self.y + 1) as f64)
            .unwrap();
        self.gc
            .draw_image_with_image_bitmap(image, (self.x + 2) as f64, (self.y + 1) as f64)
            .unwrap();
        self.gc
            .draw_image_with_image_bitmap(image, (self.x + 1) as f64, self.y as f64)
            .unwrap();
        self.gc
            .draw_image_with_image_bitmap(image, (self.x + 1) as f64, (self.y + 2) as f64)
            .unwrap();

        // Main image
        self.gc.clear_rect(
            (self.x + 1) as f64,
            (self.y + 1) as f64,
            image.width() as f64,
            image.height() as f64,
        );

        self.gc
            .draw_image_with_image_bitmap(image, (self.x + 1) as f64, (self.y + 1) as f64)
            .unwrap();

        self.next_y = self.next_y.max(image.height() + 2 + self.y);
        self.x += image.width() + 2;

        rect
    }

    pub fn build_atlas(self) -> ImageBitmap {
        self.canvas.transfer_to_image_bitmap().unwrap()
    }
}
