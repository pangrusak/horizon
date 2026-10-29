use std::{cell::RefCell, rc::Rc, sync::Arc};

use wasm_bindgen::{JsCast, prelude::*};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys},
    window::{Window, WindowId},
};

use crate::{
    gfx::{Renderer, panorama},
    graphics::GraphicsApi,
};

fn status(message: &str) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(element) = document.get_element_by_id("status") {
            element.set_text_content(Some(message));
        }
    }
}

#[derive(Default)]
struct BrowserApp {
    window: Option<Arc<Window>>,
    renderer: Rc<RefCell<Option<Renderer>>>,
}

impl ApplicationHandler for BrowserApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let canvas = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("horizon-canvas"))
            .and_then(|element| {
                element.dyn_into::<web_sys::HtmlCanvasElement>().ok()
            });

        let Some(canvas) = canvas else {
            status("Renderer startup failed: canvas not found.");
            event_loop.exit();
            return;
        };

        let attributes = Window::default_attributes()
            .with_title("Horizon")
            .with_canvas(Some(canvas))
            .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720));

        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                status(&format!("Window creation failed: {error}"));
                event_loop.exit();
                return;
            }
        };

        self.window = Some(window.clone());
        let renderer_slot = self.renderer.clone();

        status("Winit canvas attached. Initializing Horizon renderer…");

        wasm_bindgen_futures::spawn_local(async move {
            let white = [255u8, 255, 255, 255];
            let transparent = [0u8; 4];
            let black = [0u8, 0, 0, 255];

            let panorama = panorama::Faces {
                size: 1,
                pixels: black.repeat(6),
            };

            let result = Renderer::new(
                window,
                false,
                &white,
                1,
                1,
                1,
                &[],
                &[],
                (&white, 1, 1),
                (&transparent, 1, 1),
                (&transparent, 1, 1),
                (&transparent, 1),
                &panorama,
                None,
                None,
                GraphicsApi::Auto,
            )
            .await;

            match result {
                Ok(renderer) => {
                    *renderer_slot.borrow_mut() = Some(renderer);
                    status(
                        "Horizon Renderer initialized with placeholder textures.\n\
                         Renderer retained; no game frame drawn yet.",
                    );
                }
                Err(error) => {
                    status(&format!("Horizon renderer initialization failed: {error}"));
                }
            }
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if matches!(event, WindowEvent::CloseRequested) {
            event_loop.exit();
        }
    }
}

#[wasm_bindgen]
pub fn start_renderer() -> Result<(), JsValue> {
    let event_loop = EventLoop::new()
        .map_err(|error| JsValue::from_str(&error.to_string()))?;

    event_loop.spawn_app(BrowserApp::default());
    Ok(())
}