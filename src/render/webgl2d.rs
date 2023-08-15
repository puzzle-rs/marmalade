use glam::{Mat3, UVec2, Vec3};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, ImageBitmap, OffscreenCanvas, WebGl2RenderingContext, WebGlBuffer,
    WebGlTexture, WebGlUniformLocation,
};

use crate::global::window;

use super::{
    object2d::Drawable2D,
    webgl_util::{buffer_f32_slice, compile_shader, link_program},
    Color,
};

pub struct Webgl2d {
    canvas: OffscreenCanvas,
    webgl: WebGl2RenderingContext,
    position_buffer: WebGlBuffer,
    color_buffer: WebGlBuffer,
    texcoord_buffer: WebGlBuffer,
    view_matrix_uniform_location: WebGlUniformLocation,
    white_texture: WebGlTexture,
    view_matrix: Mat3,
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
            r##"#version 300 es
        
                in vec2 aPosition;
                in vec2 aTexcoord;
                in vec4 aColor;

                uniform mat3 uViewMatrix;

                out vec4 vColor;
                out vec2 vTexcoord;

                void main() {
                    gl_Position = vec4((uViewMatrix * vec3(aPosition, 1.)).xy, 1., 1.);

                    vColor = aColor;
                    vTexcoord = aTexcoord;
                }"##,
        );

        let frag_shader = compile_shader(
            &webgl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r##"#version 300 es
            
                precision highp float;

                in vec4 vColor;
                in vec2 vTexcoord;

                uniform sampler2D uTexture;

                out vec4 outColor;
                
                void main() {
                    outColor = texture(uTexture, vTexcoord) * vColor;
                }"##,
        );

        let program = link_program(&webgl, &vert_shader, &frag_shader);

        webgl.use_program(Some(&program));

        let position_attribute_location = webgl.get_attrib_location(&program, "aPosition");
        //assert!(position_attribute_location > -1);

        let color_attribute_location = webgl.get_attrib_location(&program, "aColor");
        //assert!(color_attribute_location > -1);

        let texcoord_attribute_location = webgl.get_attrib_location(&program, "aTexcoord");
        //assert!(texcoord_attribute_location > -1);

        let image_uniform_location = webgl
            .get_uniform_location(&program, "uTexture")
            .expect("Can't get texture location");

        let view_matrix_uniform_location = webgl
            .get_uniform_location(&program, "uViewMatrix")
            .expect("Can't get view matrix location");

        // Create a white texture inside texture unit 1 for later use
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

        // TODO set the

        //let vao = webgl.create_vertex_array().expect("Can't create VAO");
        //webgl.bind_vertex_array(Some(&vao));

        let position_buffer = webgl.create_buffer().expect("Failed to create buffer");
        webgl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));
        webgl.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        webgl.enable_vertex_attrib_array(position_attribute_location as u32);

        let color_buffer = webgl.create_buffer().expect("Failed to create buffer");
        webgl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
        webgl.vertex_attrib_pointer_with_i32(
            color_attribute_location as u32,
            4,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        webgl.enable_vertex_attrib_array(color_attribute_location as u32);

        let texcoord_buffer = webgl.create_buffer().expect("Failed to create buffer");
        webgl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&texcoord_buffer));
        webgl.vertex_attrib_pointer_with_i32(
            texcoord_attribute_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        webgl.enable_vertex_attrib_array(texcoord_attribute_location as u32);

        webgl.enable(WebGl2RenderingContext::BLEND);
        webgl.blend_func(
            WebGl2RenderingContext::SRC_ALPHA,
            WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        Self {
            canvas,
            webgl,
            position_buffer,
            color_buffer,
            texcoord_buffer,
            view_matrix_uniform_location,
            white_texture,
            view_matrix: Mat3::IDENTITY,
        }
    }

    pub fn draw<T: Drawable2D>(&self, drawable: &T) {
        let data = drawable.checked_get_draw_data();

        buffer_f32_slice(&self.webgl, &self.position_buffer, data.positions);
        buffer_f32_slice(&self.webgl, &self.color_buffer, data.colors);
        buffer_f32_slice(&self.webgl, &self.texcoord_buffer, data.texcoords);

        self.webgl.uniform_matrix3fv_with_f32_array(
            Some(&self.view_matrix_uniform_location),
            false,
            &self.view_matrix.to_cols_array(),
        );

        self.webgl.bind_texture(
            WebGl2RenderingContext::TEXTURE_2D,
            Some(if let Some(texture) = data.texture {
                texture
            } else {
                &self.white_texture
            }),
        );

        self.webgl.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            (data.positions.len() / 2) as i32,
        );
    }

    pub fn set_view_matrix(&mut self, view_matrix: Mat3) {
        self.view_matrix = view_matrix;
    }

    /// Set the view matrix so that world coordinates corresponds to pixel on the canvas
    pub fn pixel_perfect_view(&mut self) {
        self.view_matrix = Mat3::from_cols(
            Vec3::new(2. / self.canvas.width() as f32, 0., 0.),
            Vec3::new(0., 2. / self.canvas.height() as f32, 0.),
            Vec3::new(-1., -1., 1.),
        );
    }

    #[must_use]
    pub fn create_texture(&self, image: &ImageBitmap) -> WebGlTexture {
        let texture = self.webgl.create_texture().expect("Can't create texture");
        self.webgl
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        self.webgl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_BASE_LEVEL,
            0,
        );

        self.webgl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAX_LEVEL,
            0,
        );

        self.webgl
            .tex_image_2d_with_u32_and_u32_and_image_bitmap(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                image,
            )
            .expect("Can't upload image to gpu");

        texture
    }

    pub fn clear(&self, clear_color: Color) {
        let c = clear_color.f32_color();

        self.webgl.clear_color(c.x, c.y, c.z, c.w);
        self.webgl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn set_size(&self, size: UVec2) {
        if size.x != self.canvas.width() || size.y != self.canvas.height() {
            self.canvas.set_width(size.x);
            self.canvas.set_height(size.y);

            self.webgl.viewport(0, 0, size.x as i32, size.y as i32);
        }
    }

    pub fn fit_screen(&self) {
        let window = window();

        self.set_size(UVec2::new(
            window.inner_width().unwrap().as_f64().unwrap() as u32,
            window.inner_height().unwrap().as_f64().unwrap() as u32,
        ));
    }
}
