// #![deny(unused_crate_dependencies)]

//! Main Application file

mod renderer;
mod geometry;
mod performance_monitor;
mod textured_quad;
mod counter_gui;
mod counter;
mod counter_gui_subview;
mod test_examples;


use counter::Message;
use test_examples::ExampleTests;
use wgpu_gui::{core::{gui_functions::GuiElement, gui_message::GuiMessage, mouse_event::MouseEvent}, wgpu::wgpu_widget_renderer::{WgpuWidgetRenderer, WgpuWidgetRendererStorage}, widget::widget_renderer};
use wgpu_renderer::default_window;
use winit::event::{ElementState, MouseButton, TouchPhase, WindowEvent};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;


struct WgpuGuiExample<'a>{
    scale_factor: f32,

    renderer: renderer::Renderer<'a>,
    performance_monitor: performance_monitor::PerformanceMonitor,

    // data
    textured_quad: textured_quad::TexturedQuad,

    // gui
    widget_renderer_storage: WgpuWidgetRendererStorage,
    font: rusttype::Font<'static>,

    mouse_event: MouseEvent,
    counter1: counter::Counter,
    counter2: counter::Counter,
    counter_gui: counter_gui::CounterGui,

    // example tests
    example_tests: ExampleTests,
}

impl<'a> WgpuGuiExample<'a> {
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

        // font 
        let font_data = include_bytes!("../../wgpu_renderer/src/freefont/FreeMono.ttf");
        let font = rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

        // gui
        let mut widget_renderer_storage = WgpuWidgetRendererStorage::new();
        let mut widget_renderer = WgpuWidgetRenderer {
            storage: &mut widget_renderer_storage,
            font: &font,
            wgpu_renderer: &mut renderer.wgpu_renderer,
            texture_bind_group_layout: &renderer.texture_bind_group_layout,
        };

        let mouse_event = MouseEvent::new();
        let counter1 = counter::Counter::new();
        let counter2 = counter::Counter::new();
        let counter_gui = counter_gui::CounterGui::new(&mut widget_renderer);

        let example_tests = ExampleTests::new(&mut widget_renderer);
        
        Self {
            scale_factor,

            renderer,
            performance_monitor,

            textured_quad,

            widget_renderer_storage,
            font,

            mouse_event,
            counter1,
            counter2,
            counter_gui,

            example_tests,
        }
    }



    fn handle_gui_mouse_pressed(&mut self, is_pressed: bool) -> bool {
        self.mouse_event.is_pressed = is_pressed;
        self.handle_gui_event()
    }

    fn handle_gui_mouse_moved(&mut self, x: u32, y: u32) -> bool {
        self.mouse_event.x = x;
        self.mouse_event.y = y;
        self.handle_gui_event()
    }
    
    fn handle_gui_event(&mut self) -> bool {

        // self.counter_gui.mouse_event(&self.mouse_event, &mut |message: Message| { 
        //     match message{
        //         Message::SubView1(message) => self.counter1.message(message),
        //         Message::SubView2(message) => self.counter2.message(message),
        //     }
        // });

        self.example_tests.mouse_event(&self.mouse_event, &mut |_message| {
            println!("{:?}", _message);
        });

        true
    }

}

#[allow(unused)]
// fn apply_scale_factor(position: winit::dpi::PhysicalPosition<f64>, scale_factor: f32) 
// -> winit::dpi::PhysicalPosition<f64> 
// {
//     cfg_if::cfg_if! {
//         // apply scale factor for the web
//         if #[cfg(target_arch = "wasm32")] {
//             let mut res = position;
//             res.x = res.x / scale_factor as f64;
//             res.y = res.y / scale_factor as f64;
//             res
//         }
//         else {
//             position
//         }
//     }
// }

impl<'a> default_window::DefaultWindowApp for WgpuGuiExample<'a>
{
    fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.renderer.size()
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);

        let size = wgpu_gui::core::size::Size{width: new_size.width, height: new_size.height};

        let mut widget_renderer = WgpuWidgetRenderer {
            storage: &mut self.widget_renderer_storage,
            font: &self.font,
            wgpu_renderer: &mut self.renderer.wgpu_renderer,
            texture_bind_group_layout: &self.renderer.texture_bind_group_layout,
        };

        // self.counter_gui.resize(0, 0, size);
        self.example_tests.resize(&mut widget_renderer, 0, 0, size);
    }

    fn update_scale_factor(&mut self, scale_factor: f32) {
        self.scale_factor = scale_factor;
    }

    fn update(&mut self, dt: instant::Duration) {
        self.renderer.update(dt);

        self.performance_monitor.update(&mut self.renderer.wgpu_renderer);
    
        // self.counter_gui.update(&self.counter1, &self.counter2);
        // self.counter_gui.update_device();

        self.example_tests.update_device(&mut self.renderer.wgpu_renderer);
        // self.example_tests.draw();

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
                    let is_pressed = *state == ElementState::Pressed;
                    
                    let _res = self.handle_gui_mouse_pressed(is_pressed);

                    true
                } 
                WindowEvent::CursorMoved { position, .. } => {
                    // let _pos = apply_scale_factor(*position, self.scale_factor);

                    let _res = self.handle_gui_mouse_moved(position.x as u32, position.y as u32);

                    true
                },
                WindowEvent::Touch(touch) => {
                    // let _pos = apply_scale_factor(touch.location, self.scale_factor);
    
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
            &mut self.performance_monitor,
            &mut self.widget_renderer_storage,    
            )
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

    let app = WgpuGuiExample::new(&window).await;
    default_window::run(event_loop, &window, app);
}