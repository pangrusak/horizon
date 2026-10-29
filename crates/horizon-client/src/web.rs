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
#[wasm_bindgen]
pub async fn probe_gpu() -> Result<String, JsValue> {
    crate::gfx::browser_device_probe()
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen]
pub async fn present_test_frame() -> Result<(), JsValue> {
    present_test_frame_inner()
        .await
        .map_err(|error| JsValue::from_str(&error))
}

async fn present_test_frame_inner() -> Result<(), String> {
    let window = web_sys::window().ok_or("Missing browser window")?;
    let document = window.document().ok_or("Missing document")?;

    let canvas = document
        .get_element_by_id("horizon-canvas")
        .ok_or("Missing #horizon-canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| "Element is not a canvas".to_owned())?;

    let width = canvas.width().max(1);
    let height = canvas.height().max(1);

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });

    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
        .map_err(|error| error.to_string())?;

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            ..Default::default()
        })
        .await
        .map_err(|error| error.to_string())?;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Horizon canvas test"),
            ..Default::default()
        })
        .await
        .map_err(|error| error.to_string())?;

    let config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No supported canvas surface configuration")?;

    surface.configure(&device, &config);

    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,

        status => {
            return Err(format!(
                "Could not acquire the canvas frame: {status:?}"
            ));
        }
    };

    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Horizon canvas test encoder"),
        });

    {
        let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Horizon canvas test clear"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.55,
                        b: 0.35,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }

    queue.submit(Some(encoder.finish()));
    queue.present(frame);

    Ok(())
}