use std::f32::consts::TAU;

use glam::Vec2;
use web_sys::WebGlTexture;

use super::Color;

pub trait Drawable2D {
    /// Get the necessary data for drawing the object
    fn get_draw_data(&self) -> DrawData2D;

    /// Get the necessary data for drawing the object, but check if buffer have correct sizes
    fn checked_get_draw_data(&self) -> DrawData2D {
        let data = self.get_draw_data();

        assert_eq!(data.positions.len(), data.texcoords.len());
        assert_eq!(data.positions.len() * 2, data.colors.len());

        data
    }
}

pub struct DrawData2D<'a> {
    pub positions: &'a [f32],
    pub colors: &'a [f32],
    pub texcoords: &'a [f32],
    pub texture: Option<&'a WebGlTexture>,
}

impl<'a> Drawable2D for DrawData2D<'a> {
    fn get_draw_data(&self) -> DrawData2D<'a> {
        DrawData2D {
            positions: self.positions,
            colors: self.colors,
            texcoords: self.texcoords,
            texture: self.texture,
        }
    }
}

pub struct Rect2D {
    points: [f32; 12],
    colors: [f32; 24],
    texcoords: [f32; 12],
    texture: Option<WebGlTexture>,
}

impl Drawable2D for Rect2D {
    fn get_draw_data(&self) -> DrawData2D {
        DrawData2D {
            positions: &self.points,
            colors: &self.colors,
            texcoords: &self.texcoords,
            texture: if let Some(ref texture) = self.texture {
                Some(texture)
            } else {
                None
            },
        }
    }
}

impl Rect2D {
    #[must_use]
    pub fn new_colored(pos: Vec2, size: Vec2, color: Color) -> Self {
        let x = pos.x;
        let y = pos.y;

        let w = size.x;
        let h = size.y;

        let c = color.f32_color();

        let r = c.x;
        let g = c.y;
        let b = c.z;
        let a = c.w;

        Self {
            points: [x, y, x + w, y, x, y + h, x + w, y, x, y + h, x + w, y + h],
            colors: [
                r, g, b, a, r, g, b, a, r, g, b, a, r, g, b, a, r, g, b, a, r, g, b, a,
            ],
            texcoords: [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
            texture: None,
        }
    }

    #[must_use]
    pub fn new_textured(pos: Vec2, size: Vec2, texture: WebGlTexture) -> Self {
        let x = pos.x;
        let y = pos.y;

        let w = size.x;
        let h = size.y;

        Self {
            points: [x, y, x + w, y, x, y + h, x + w, y, x, y + h, x + w, y + h],
            colors: [
                1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
                1., 1., 1.,
            ],
            texcoords: [0., 1., 1., 1., 0., 0., 1., 1., 0., 0., 1., 0.],
            texture: Some(texture),
        }
    }
}

pub struct Circle2D {
    points: Vec<f32>,
    colors: Vec<f32>,
    texcoords: Vec<f32>,
    texture: Option<WebGlTexture>,
}

impl Drawable2D for Circle2D {
    fn get_draw_data(&self) -> DrawData2D {
        DrawData2D {
            positions: &self.points,
            colors: &self.colors,
            texcoords: &self.texcoords,
            texture: if let Some(ref texture) = self.texture {
                Some(texture)
            } else {
                None
            },
        }
    }
}

impl Circle2D {
    #[must_use]
    pub fn new_colored(center: Vec2, radius: f32, sides: u16, color: Color) -> Self {
        let color = color.f32_color();

        let mut points = Vec::new();
        let mut colors = Vec::new();
        let texcoords = vec![0.; sides as usize * 6];

        for i in 0..sides {
            let x1 = (i as f32 * TAU / sides as f32).cos();
            let y1 = (i as f32 * TAU / sides as f32).sin();

            let x2 = ((i + 1) as f32 * TAU / sides as f32).cos();
            let y2 = ((i + 1) as f32 * TAU / sides as f32).sin();

            points.push(center.x);
            points.push(center.y);
            points.push(x1.mul_add(radius, center.x));
            points.push(y1.mul_add(radius, center.y));
            points.push(x2.mul_add(radius, center.x));
            points.push(y2.mul_add(radius, center.y));

            for _ in 0..3 {
                colors.push(color.x);
                colors.push(color.y);
                colors.push(color.z);
                colors.push(color.w);
            }
        }

        Self {
            points,
            colors,
            texcoords,
            texture: None,
        }
    }

    #[must_use]
    pub fn new_textured(center: Vec2, radius: f32, sides: u16, texture: WebGlTexture) -> Self {
        let mut points = Vec::new();
        let colors = vec![1.; sides as usize * 12];
        let mut texcoords = Vec::new();

        for i in 0..sides {
            let x1 = (i as f32 * TAU / sides as f32).cos();
            let y1 = (i as f32 * TAU / sides as f32).sin();

            let x2 = ((i + 1) as f32 * TAU / sides as f32).cos();
            let y2 = ((i + 1) as f32 * TAU / sides as f32).sin();

            points.push(center.x);
            points.push(center.y);
            points.push(x1.mul_add(radius, center.x));
            points.push(y1.mul_add(radius, center.y));
            points.push(x2.mul_add(radius, center.x));
            points.push(y2.mul_add(radius, center.y));

            texcoords.push(0.5);
            texcoords.push(0.5);
            texcoords.push((x1 + 1.) / 2.);
            texcoords.push((1. - y1) / 2.);
            texcoords.push((x2 + 1.) / 2.);
            texcoords.push((1. - y2) / 2.);
        }

        Self {
            points,
            colors,
            texcoords,
            texture: Some(texture),
        }
    }
}
