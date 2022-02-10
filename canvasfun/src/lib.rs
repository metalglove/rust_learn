use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use random_color::RandomColor;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let canvas: web_sys::HtmlCanvasElement = get_canvas();
    let context: web_sys::CanvasRenderingContext2d = get_rendering_context(&canvas);
    
    let context = Rc::new(context);
    // scoped context for mousedown handler
    {
        let context = context.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let offset_x = event.offset_x() as f64;
            let offset_y = event.offset_y() as f64;
            draw_smiley(&context, offset_x, offset_y);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // scoped context for mouseup handler 
    {
        let context = context.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let offset_x = event.offset_x() as f64;
            let offset_y = event.offset_y() as f64;
            
            context.line_to(offset_x, offset_y);
            context.stroke();

            draw_smiley(&context, offset_x, offset_y);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    return Ok(());
}

/// Gets the html canvas element from the active window.
fn get_canvas() -> web_sys::HtmlCanvasElement {
    let document: web_sys::Document = document();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Should be a HtmlCanvasElement");
    return canvas;
}

/// Gets the 2d rendering context of the canvas element.
fn get_rendering_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    let context: web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    return context;
}

/// Draws a smiley on the position of the mouse click.
fn draw_smiley(context: &web_sys::CanvasRenderingContext2d, offset_x: f64, offset_y: f64) {
    context.begin_path();

    // Draw the outer circle.
    context
        .arc(offset_x, offset_y, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(offset_x + 35.0, offset_y);
    context
        .arc(offset_x, offset_y, 35.0, 0.0, f64::consts::PI)
        .unwrap();

    // Draw the left eye.
    context.move_to(offset_x - 10.0, offset_y - 10.0);
    context
        .arc(offset_x - 15.0, offset_y - 10.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye. 
    context.move_to(offset_x + 20.0, offset_y - 10.0);
    context
        .arc(offset_x + 15.0, offset_y - 10.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.set_stroke_style(&RandomColor::new().seed(42).to_rgb_string().into());
    context.stroke();
    
    // return position to x and y offset.
    context.move_to(offset_x, offset_y);
}

/// Gets the window object.
fn window() -> web_sys::Window {
    web_sys::window()
        .expect("The global `window` does not exist.")
}

/// Gets the document field. 
fn document() -> web_sys::Document {
    window()
        .document()
        .expect("A `window` object should have a `document` field.")
}
