use std::{cell::RefCell, f32::consts::TAU};

use glam::{Mat3, UVec2, Vec2, Vec3};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, ImageBitmap, OffscreenCanvas, WebGl2RenderingContext, WebGlBuffer,
    WebGlUniformLocation,
};

use crate::global::window;

use super::{
    atlas::{TextureRect, NO_TEXTURE_RECT},
    webgl_util::{buffer_f32_slice, buffer_u16_indexes, compile_shader, link_program},
    Color,
};

pub trait DrawTarget2d {
    /// Draw on context from raw vertex data
    fn draw_raw(&mut self, indexes: &[u16], positions: &[f32], colors: &[f32], texcoords: &[f32]);

    /// Draw a rectangle, color and texture are multiplied
    fn draw_rect(&mut self, position: Vec2, size: Vec2, color: Color, texture: &TextureRect) {
        let x = position.x;
        let y = position.y;

        let w = size.x;
        let h = size.y;

        let f32_color = color.f32_color();

        let r = f32_color.x;
        let g = f32_color.y;
        let b = f32_color.z;
        let a = f32_color.w;

        let t_x = texture.position.x;
        let t_y = texture.position.y;

        let t_w = texture.size.x;
        let t_h = texture.size.y;

        self.draw_raw(
            &[0, 1, 2, 1, 2, 3],
            &[x, y, x + w, y, x, y + h, x + w, y + h],
            &[r, g, b, a, r, g, b, a, r, g, b, a, r, g, b, a],
            &[
                t_x,
                t_y + t_h,
                t_x + t_w,
                t_y + t_h,
                t_x,
                t_y,
                t_x + t_w,
                t_y,
            ],
        );
    }

    /// Draw a rectangle with given color
    fn draw_colored_rect(&mut self, position: Vec2, size: Vec2, color: Color) {
        self.draw_rect(position, size, color, &NO_TEXTURE_RECT);
    }

    /// Draw a rectangle with given texture
    fn draw_textured_rect(&mut self, position: Vec2, size: Vec2, texture: &TextureRect) {
        self.draw_rect(position, size, Color::rgb(255, 255, 255), texture);
    }

    /// Draw a regular polygon, color and texture are multiplied
    fn draw_regular(
        &mut self,
        center: Vec2,
        radius: f32,
        sides: u16,
        color: Color,
        texture: &TextureRect,
    ) {
        let c_x = center.x;
        let c_y = center.y;

        let f32_color = color.f32_color();

        let r = f32_color.x;
        let g = f32_color.y;
        let b = f32_color.z;
        let a = f32_color.w;

        let t_x = texture.position.x;
        let t_y = texture.position.y;

        let t_w = texture.size.x;
        let t_h = texture.size.y;

        let mut indexes = Vec::new();
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut texcoords = Vec::new();

        positions.push(c_x);
        positions.push(c_y);

        colors.push(r);
        colors.push(g);
        colors.push(b);
        colors.push(a);

        texcoords.push(t_x + t_w / 2.);
        texcoords.push(t_y + t_h / 2.);

        let step_size = TAU / sides as f32;

        for i in 1..=sides {
            indexes.push(0);
            indexes.push(i);
            indexes.push(if i == sides { 1 } else { i + 1 });

            let (sin_y, cos_x) = (i as f32 * step_size).sin_cos();

            positions.push(c_x + cos_x * radius);
            positions.push(c_y + sin_y * radius);

            colors.push(r);
            colors.push(g);
            colors.push(b);
            colors.push(a);

            texcoords.push(t_x + t_w * (cos_x + 1.) / 2.);
            texcoords.push(t_y + t_h * (1. - sin_y) / 2.);
        }

        self.draw_raw(&indexes, &positions, &colors, &texcoords);
    }

    /// Draw a regular polygon with given color
    fn draw_colored_regular(&mut self, center: Vec2, radius: f32, sides: u16, color: Color) {
        self.draw_regular(center, radius, sides, color, &NO_TEXTURE_RECT);
    }

    /// Draw a regular polygon with given texture
    fn draw_textured_regular(
        &mut self,
        center: Vec2,
        radius: f32,
        sides: u16,
        texture: &TextureRect,
    ) {
        self.draw_regular(center, radius, sides, Color::rgb(255, 255, 255), texture);
    }
}

/// A utility struct for easily batching geometry together
pub struct BufferBuilder2d {
    index_counter: u16,
    indexes: Vec<u16>,
    positions: Vec<f32>,
    colors: Vec<f32>,
    texcoords: Vec<f32>,
}

/// Buffer builder is used to upload geometry data to the gpu in advance and redraw it without having to reupload it
impl BufferBuilder2d {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            index_counter: 0,
            indexes: Vec::new(),
            positions: Vec::new(),
            colors: Vec::new(),
            texcoords: Vec::new(),
        }
    }
}

impl DrawTarget2d for BufferBuilder2d {
    fn draw_raw(&mut self, indexes: &[u16], positions: &[f32], colors: &[f32], texcoords: &[f32]) {
        assert_eq!(positions.len(), texcoords.len());
        assert_eq!(positions.len() * 2, colors.len());

        let mut increment = 0;

        for &i in indexes {
            increment = increment.max(i);
            self.indexes.push(i + self.index_counter);
        }

        self.index_counter = self
            .index_counter
            .checked_add(increment + 1)
            .expect("Error, buffers are limited to 65536 vertices");

        self.positions.extend_from_slice(positions);
        self.colors.extend_from_slice(colors);
        self.texcoords.extend_from_slice(texcoords);
    }
}

/// A buffer of geometry ready to be drawn to the screen.
/// Can be reused multiple times efficiently without having to rebuild one
pub struct Buffer2d {
    count: u16,
    index_buffer: WebGlBuffer,
    position_buffer: WebGlBuffer,
    color_buffer: WebGlBuffer,
    texcoord_buffer: WebGlBuffer,
}

/// A accelerated 2d drawing context backed by webgl2
pub struct Webgl2d {
    canvas: OffscreenCanvas,
    gl: WebGl2RenderingContext,
    position_attribute_location: i32,
    color_attribute_location: i32,
    texcoord_attribute_location: i32,
    view_matrix_uniform_location: WebGlUniformLocation,
    view_matrix: Mat3,
    direct_draw_builder: RefCell<BufferBuilder2d>,
}

impl Webgl2d {
    #[must_use]
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        let canvas = canvas.transfer_control_to_offscreen().unwrap();

        Self::internal_new(canvas)
    }

    #[must_use]
    pub fn new_offscreen(size: UVec2) -> Self {
        let canvas = OffscreenCanvas::new(size.x, size.y).unwrap();

        Self::internal_new(canvas)
    }

    #[must_use]
    fn internal_new(canvas: OffscreenCanvas) -> Self {
        let webgl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        let vert_shader = compile_shader(
            &webgl,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("webgl2d.vert"),
        );

        let frag_shader = compile_shader(
            &webgl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("webgl2d.frag"),
        );

        let program = link_program(&webgl, &vert_shader, &frag_shader);

        webgl.use_program(Some(&program));

        let position_attribute_location = webgl.get_attrib_location(&program, "aPosition");

        let color_attribute_location = webgl.get_attrib_location(&program, "aColor");

        let texcoord_attribute_location = webgl.get_attrib_location(&program, "aTexcoord");

        let image_uniform_location = webgl
            .get_uniform_location(&program, "uTexture")
            .expect("Can't get texture location");

        let view_matrix_uniform_location = webgl
            .get_uniform_location(&program, "uViewMatrix")
            .expect("Can't get view matrix location");

        let white_texture = webgl.create_texture().expect("Can't create texture");

        webgl.active_texture(WebGl2RenderingContext::TEXTURE0);
        webgl.uniform1i(Some(&image_uniform_location), 0);

        webgl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&white_texture));

        webgl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                1,
                1,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &[255, 255, 255, 255],
                0,
            )
            .expect("Can't upload data to texture");

        webgl.enable(WebGl2RenderingContext::BLEND);
        webgl.blend_func(
            WebGl2RenderingContext::ONE,
            WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        webgl.enable_vertex_attrib_array(position_attribute_location as u32);
        webgl.enable_vertex_attrib_array(color_attribute_location as u32);
        webgl.enable_vertex_attrib_array(texcoord_attribute_location as u32);

        Self {
            canvas,
            gl: webgl,
            position_attribute_location,
            color_attribute_location,
            texcoord_attribute_location,
            view_matrix_uniform_location,
            view_matrix: Mat3::IDENTITY,
            direct_draw_builder: RefCell::new(BufferBuilder2d::new()),
        }
    }

    /// Draw the given buffer on the canvas.
    /// It may be necessary to flush draw calls done without a buffer before drawing this buffer, it is however never needed to flush after drawing a buffer.
    pub fn draw_buffer(&self, buffer: &Buffer2d) {
        self.gl.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&buffer.index_buffer),
        );

        self.gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&buffer.position_buffer),
        );
        self.gl.vertex_attrib_pointer_with_i32(
            self.position_attribute_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        self.gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&buffer.color_buffer),
        );
        self.gl.vertex_attrib_pointer_with_i32(
            self.color_attribute_location as u32,
            4,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        self.gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&buffer.texcoord_buffer),
        );
        self.gl.vertex_attrib_pointer_with_i32(
            self.texcoord_attribute_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        self.gl.uniform_matrix3fv_with_f32_array(
            Some(&self.view_matrix_uniform_location),
            false,
            &self.view_matrix.to_cols_array(),
        );

        self.gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            buffer.count as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    /// Create the opengl buffers from the data inside the builder
    /// After this operation the builder is emptied and ca be reused for building different geometry
    /// (Reusing it is more efficient since it can reuse some of its previous memory)
    pub fn build_buffer(&self, buffer: &mut BufferBuilder2d) -> Buffer2d {
        let index_buffer: WebGlBuffer = self.gl.create_buffer().expect("Failed to create buffer");
        buffer_u16_indexes(&self.gl, &index_buffer, &buffer.indexes);

        let position_buffer = self.gl.create_buffer().expect("Failed to create buffer");
        buffer_f32_slice(&self.gl, &position_buffer, &buffer.positions);

        let color_buffer = self.gl.create_buffer().expect("Failed to create buffer");
        buffer_f32_slice(&self.gl, &color_buffer, &buffer.colors);

        let texcoord_buffer = self.gl.create_buffer().expect("Failed to create buffer");
        buffer_f32_slice(&self.gl, &texcoord_buffer, &buffer.texcoords);

        let count = u16::try_from(buffer.indexes.len())
            .expect("Error, buffers are limited to 65536 vertices");

        buffer.index_counter = 0;
        buffer.indexes.clear();
        buffer.positions.clear();
        buffer.colors.clear();
        buffer.texcoords.clear();

        Buffer2d {
            count,
            index_buffer,
            position_buffer,
            color_buffer,
            texcoord_buffer,
        }
    }

    /// Set the view matrix of this context, is is used to convert from world coordinates to opengl coordinates
    pub fn set_view_matrix(&mut self, view_matrix: Mat3) {
        self.view_matrix = view_matrix;
    }

    /// Set the view matrix so that world coordinates corresponds to pixels on the canvas
    pub fn pixel_perfect_view(&mut self) {
        self.view_matrix = Mat3::from_cols(
            Vec3::new(2. / self.canvas.width() as f32, 0., 0.),
            Vec3::new(0., 2. / self.canvas.height() as f32, 0.),
            Vec3::new(-1., -1., 1.),
        );
    }

    /// Set the view matrix to a camera centered at `cam_pos` which can see at a distance `view_radius` on the left and right
    /// Height view distance is adjusted so that there is no stretch on the vertical axis
    pub fn camera_view(&mut self, cam_pos: Vec2, view_radius: f32) {
        let height_factor = self.canvas.width() as f32 / self.canvas.height() as f32;

        self.view_matrix = Mat3::from_cols(
            Vec3::new(1. / view_radius, 0., 0.),
            Vec3::new(0., height_factor / view_radius, 0.),
            Vec3::new(
                -cam_pos.x / view_radius,
                -height_factor * cam_pos.y / view_radius,
                1.,
            ),
        );
    }

    /// Computes the world coordinates corresponding to the given screen coordinates with the current view matrix
    pub fn screen_to_world_pos(&self, screen_pos: Vec2) -> Vec2 {
        let screen_to_ogl_matrix = Mat3::from_cols(
            Vec3::new(2. / self.canvas.width() as f32, 0., 0.),
            Vec3::new(0., -2. / self.canvas.height() as f32, 0.),
            Vec3::new(-1., 1., 1.),
        );

        self.view_matrix
            .inverse()
            .transform_point2(screen_to_ogl_matrix.transform_point2(screen_pos))
    }

    /// Computes the screen coordinates corresponding to the given world coordinates with the current view matrix
    pub fn world_to_screen_pos(&self, world_pos: Vec2) -> Vec2 {
        let screen_to_ogl_matrix = Mat3::from_cols(
            Vec3::new(2. / self.canvas.width() as f32, 0., 0.),
            Vec3::new(0., -2. / self.canvas.height() as f32, 0.),
            Vec3::new(-1., 1., 1.),
        );

        screen_to_ogl_matrix
            .inverse()
            .transform_point2(self.view_matrix.transform_point2(world_pos))
    }

    /// Set the texture that will be used by the graphic context, it is usually built with a `AtlasBuilder`
    pub fn set_texture(&self, image: &ImageBitmap) {
        let texture = self.gl.create_texture().expect("Can't create texture");
        self.gl
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        self.gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32,
        );

        self.gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );

        self.gl
            .tex_image_2d_with_u32_and_u32_and_image_bitmap(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                image,
            )
            .expect("Can't upload image to gpu");

        self.gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
    }

    /// Clear canvas with the given color
    pub fn clear(&self, clear_color: Color) {
        let c = clear_color.f32_color();

        self.gl.clear_color(c.x, c.y, c.z, c.w);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    /// Set the size of this canvas
    pub fn set_size(&self, size: UVec2) {
        if size.x != self.canvas.width() || size.y != self.canvas.height() {
            self.canvas.set_width(size.x);
            self.canvas.set_height(size.y);

            self.gl.viewport(0, 0, size.x as i32, size.y as i32);
        }
    }

    /// Set the size of the canvas to the window
    pub fn fit_screen(&self) {
        let window = window();

        self.set_size(UVec2::new(
            window.inner_width().unwrap().as_f64().unwrap() as u32,
            window.inner_height().unwrap().as_f64().unwrap() as u32,
        ));
    }

    /// Flush the internal draw buffers, this should be called after drawing each frame to ensure changes are displayed
    pub fn flush(&mut self) {
        let buffer = self.build_buffer(&mut self.direct_draw_builder.borrow_mut());

        self.draw_buffer(&buffer);
    }
}

impl DrawTarget2d for Webgl2d {
    fn draw_raw(&mut self, indexes: &[u16], positions: &[f32], colors: &[f32], texcoords: &[f32]) {
        if self.direct_draw_builder.borrow().indexes.len() + indexes.len() > u16::MAX as usize {
            self.flush();
        }

        self.direct_draw_builder
            .borrow_mut()
            .draw_raw(indexes, positions, colors, texcoords);
    }
}