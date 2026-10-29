use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("Horizon requires a browser window"))?;

    let document = window.document()
        .ok_or_else(|| JsValue::from_str("Browser document is unavailable"))?;

    let canvas = document
        .get_element_by_id("horizon-canvas")
        .ok_or_else(|| JsValue::from_str("Missing #horizon-canvas"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_width(1280);
    canvas.set_height(720);

    let status = document
        .get_element_by_id("status")
        .ok_or_else(|| JsValue::from_str("Missing #status"))?;

    status.set_text_content(Some(
        "Horizon WASM initialized. Canvas found. Renderer not started.",
    ));

    Ok(())
}