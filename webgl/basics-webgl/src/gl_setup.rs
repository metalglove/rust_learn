use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, WebGl2RenderingContext,HtmlCanvasElement};

pub fn initialize_webgl_context() -> Result<WebGl2RenderingContext, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    gl.enable(WebGl2RenderingContext::BLEND);
    gl.blend_func(
        WebGl2RenderingContext::SRC_ALPHA,
        WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
    );
    gl.clear_color(0.0, 0.0, 0.0, 1.0); // RGBA
    gl.clear_depth(1.);

    Ok(gl)
}
