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
        let redraw_window = window.clone();

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
                        "Horizon Renderer initialized.\n\
                         Requesting the first renderer frame…",
                    );

                    redraw_window.request_redraw();
                }
                Err(error) => {
                    status(&format!(
                        "Horizon renderer initialization failed: {error}"
                    ));
                }
            }
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                let mut slot = self.renderer.borrow_mut();

                let Some(renderer) = slot.as_mut() else {
                    return;
                };

                let size = window.inner_size();
                let hud = crate::hud::HudBatch::new([
                    size.width.max(1),
                    size.height.max(1),
                ]);

                let frame = browser_frame(&hud);

                match renderer.render(&frame) {
                    Ok(true) => {
                        status(
                            "Horizon Renderer rendered and presented a title frame.\n\
                            Placeholder panorama; no assets or gameplay loaded.",
                        );
                    }

                    Ok(false) => {
                        status("Frame skipped; requesting another redraw.");
                        window.request_redraw();
                    }

                    Err(error) => {
                        status(&format!("Horizon frame rendering failed: {error}"));
                    }
                }
            }

            _ => {}
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
fn browser_frame(hud: &crate::hud::HudBatch) -> crate::gfx::Frame<'_> {
    crate::gfx::Frame {
        weather: &[],
        weather_time: crate::weather::Time::new(0, 0.0),
        camera: glam::DVec3::new(0.5, 100.0, 0.5),
        yaw: 0.0,
        pitch: 10.0,
        fov_degrees: 85.0,
        bob: glam::Mat4::IDENTITY,
        spin: glam::Mat4::IDENTITY,
        sky: crate::environment::Sky::default(),
        distance: 12.0,
        detail_distance: 12.0,
        depth_far: 1000.0,
        underwater: false,
        cave_culling: false,
        title: true,
        sky_blocked: false,
        skybox: false,
        end_sky: false,
        end_sky_sprite: None,
        environment_fog: None,
        water_fog: [0.0; 3],
        fog: None,
        dark_disc: false,
        outline: Vec::new(),
        outline_after_terrain: false,
        outline_high_contrast: false,
        debug_lines: &[],
        debug_lines_on_top: &[],
        debug_crosshair: &[],
        hud,
        glint_alpha: 0.75,
        glint_ticks: 0.0,
        nether_lighting: false,
        entities: &[],
        entity_spans: &[],
        glowing: &[],
        dynamic_lights: &[],
        clouds: None,
        lightning: Default::default(),
        additive: &[],
        see_through: &[],
        celestial: &[],
        stars: None,
        hand: &[],
        hand_fov_degrees: 70.0,
        screen: &[],
        pack: Default::default(),
        border: None,
        game_time: 0.0,
        shader_game_time: 0.0,
        end_portal_sprites: [[0.0; 4]; 2],
        crumbling_sprite: [0.0; 4],
        ticks: 0,
        darkness: (0.0, 0.0),
        night_vision: 0.0,
        block_flicker: 0.0,
        boss_darken: 0.0,
        menu_blur: None,
    }
}