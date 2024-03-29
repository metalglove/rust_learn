use super::super::common_functions as cf;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

#[allow(dead_code)]
pub struct Color2D {
    program: WebGlProgram,
    rect_vertices_array_length: usize,
    rect_vertices_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

#[allow(dead_code)]
impl Color2D {
    pub fn new(gl: &WebGl2RenderingContext) -> Self {
        let program = cf::link_program(
            &gl,
            super::super::shaders::vertex::color_2d::SHADER,
            super::super::shaders::fragment::color_2d::SHADER,
        ).unwrap();

        let vertices_rect: [f32; 12] = [
            0., 1., // x, y
            0., 0., // x, y
            1., 1., // x, y
            1., 1., // x, y
            0., 0., // x, y
            1., 0., // x, y
        ];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vertices_location = vertices_rect.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + vertices_rect.len() as u32,
        );
        let buffer_rect = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer_rect));
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        Self {
            u_color: gl.get_uniform_location(&program, "u_color").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "u_opacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "u_transform").unwrap(),
            rect_vertices_array_length: vertices_rect.len(),
            rect_vertices_buffer: buffer_rect,
            program: program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGl2RenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.rect_vertices_buffer),
        );
        gl.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(
            Some(&self.u_color),
            0.0, // r
            0.5, // g
            0.5, // b
            1.0, // a
        );

        gl.uniform1f(Some(&self.u_opacity), 1.);

        let translation_mat = cf::translation_matrix(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_height - 1.,
            0.0,
        );

        let scale_mat = cf::scaling_matrix(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_height,
            0.0,
        );

        let transform_mat = cf::mult_matrix_4(scale_mat, translation_mat);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            (self.rect_vertices_array_length / 2) as i32,
        );
    }
}
