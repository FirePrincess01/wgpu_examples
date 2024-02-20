// #![deny(unused_crate_dependencies)]

//! Main Application file

mod renderer;
mod geometry;
mod performance_monitor;
mod textured_quad;


use wgpu_renderer::default_window;
use winit::event::{ElementState, MouseButton, TouchPhase, WindowEvent};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;


struct WgpuRendererExample<'a>{
    scale_factor: f32,

    renderer: renderer::Renderer<'a>,
    performance_monitor: performance_monitor::PerformanceMonitor,

    // data
    textured_quad: textured_quad::TexturedQuad,
}

impl<'a> WgpuRendererExample<'a> {
    pub async fn new(window: &'a winit::window::Window) -> Self 
    {
        let scale_factor = window.scale_factor() as f32;

        let mut renderer = renderer::Renderer::new(window).await;
        let performance_monitor = performance_monitor::PerformanceMonitor::new(
            &mut renderer.wgpu_renderer);

        // data
        let textured_quad = textured_quad::TexturedQuad::new(
            &mut renderer.wgpu_renderer, 
            &renderer.texture_bind_group_layout);
        
        Self {
            scale_factor,

            renderer,
            performance_monitor,

            textured_quad,
        }
    }
}

#[allow(unused)]
fn apply_scale_factor(position: winit::dpi::PhysicalPosition<f64>, scale_factor: f32) 
-> winit::dpi::PhysicalPosition<f64> 
{
    cfg_if::cfg_if! {
        // apply scale factor for the web
        if #[cfg(target_arch = "wasm32")] {
            let mut res = position;
            res.x = res.x / scale_factor as f64;
            res.y = res.y / scale_factor as f64;
            res
        }
        else {
            position
        }
    }
}

impl<'a> default_window::DefaultWindowApp for WgpuRendererExample<'a>
{
    fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.renderer.size()
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    fn update_scale_factor(&mut self, scale_factor: f32) {
        self.scale_factor = scale_factor;
    }

    fn update(&mut self, dt: instant::Duration) {
        self.renderer.update(dt);

        self.performance_monitor.update(&mut self.renderer.wgpu_renderer);
    }

    fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.performance_monitor.watch.start(2);
            let res = match event {
                WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            physical_key: winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F2),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => { 
                    self.performance_monitor.show = !self.performance_monitor.show;
                    true
                },
                WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            physical_key: winit::keyboard::PhysicalKey::Code(key),
                            state,
                            ..
                        },
                    ..
                } => self.renderer.process_keyboard(*key, *state),
                WindowEvent::MouseWheel { delta, .. } => {
                    self.renderer.process_scroll(delta);
                    true
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state,//ElementState::Pressed,
                    ..
                } => {
                    let _is_pressed = *state == ElementState::Pressed;
                    
                    // let _res = self.handle_gui_mouse_pressed(is_pressed);

                    true
                } 
                WindowEvent::CursorMoved { position, .. } => {
                    let _pos = apply_scale_factor(*position, self.scale_factor);

                    // let _res = self.handle_gui_mouse_moved(pos.x as u32, pos.y as u32);

                    true
                },
                WindowEvent::Touch(touch) => {
                    let _pos = apply_scale_factor(touch.location, self.scale_factor);
    
                    match touch.phase {
                        TouchPhase::Started => {
                            // let _res = self.handle_gui_mouse_moved(pos.x as u32, pos.y as u32);
                            // let _res = self.handle_gui_mouse_pressed(true);
                        }
                        TouchPhase::Ended => {
                            // let _res = self.handle_gui_mouse_pressed(false);
                        }
                        TouchPhase::Cancelled => {
                            // let _res = self.handle_gui_mouse_pressed(false);
                        }
                        TouchPhase::Moved => {
                            // let _res = self.handle_gui_mouse_moved(pos.x as u32, pos.y as u32);
                        }
                    }
                    true
                } 
                _ => false,
            };
        self.performance_monitor.watch.stop(2);

        res
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(
            &[&self.textured_quad],
            &[],
            &mut self.performance_monitor)
    }


}




#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run()
{
    let default_window = default_window::DefaultWindow::new();
    let event_loop = default_window.event_loop;
    let window = default_window.window;

    // log::info!("log info");
    // log::warn!("log warn");
    // log::error!("log error");

    let app = WgpuRendererExample::new(&window).await;
    default_window::run(event_loop, &window, app);
}