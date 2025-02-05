use wgpu_gui::{core::{gui_functions::GuiElement, mouse_event, size::Size}, wgpu::{wgpu_widget_factory::WgpuWidgetFactory, wgpu_widget_renderer::WgpuWidgetRenderer}, widget::{button::{self, Button}, widget_factory::WidgetFactory, widget_renderer::WidgetRenderer}};
use wgpu_renderer::renderer::WgpuRendererInterface;



enum ExampleTestsKind {
    TestButton
}

pub struct ExampleTests {
    test_kind: ExampleTestsKind,
    widget_renderer: WgpuWidgetRenderer,

    test_button: TestButton,

    window_width: u32,
    window_height: u32,
}

impl ExampleTests {
    pub fn new(
        font: &rusttype::Font<'static>, 
        wgpu_renderer: &mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
        texture_bind_group_layout: &wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout,
    ) -> Self {
        let mut widget_renderer: WgpuWidgetRenderer = WgpuWidgetRenderer::new();
        let mut widget_factory = WgpuWidgetFactory::new(font, wgpu_renderer, texture_bind_group_layout, &mut widget_renderer);

        let test_button = TestButton::new(&mut widget_factory);
        
        let test_kind = ExampleTestsKind::TestButton;

        Self { 
            test_kind,
            widget_renderer,

            test_button,

            window_width: 0,
            window_height: 0,
        }
    }

    pub fn mouse_event(&mut self, mouse_event: &mouse_event::MouseEvent, model: &mut dyn FnMut(TestButtonMessage)) -> bool {
        
        let mut mouse_event_converted = mouse_event.clone();
        // mouse_event_converted.x = if self.window_width >= mouse_event_converted.x {self.window_width - mouse_event_converted.x} else {0}; 
        mouse_event_converted.y = if self.window_height >= mouse_event_converted.y {self.window_height - mouse_event_converted.y} else {0}; 
        
        match self.test_kind {
            ExampleTestsKind::TestButton => self.test_button.mouse_event(&mouse_event_converted, model),
        }   
    }

    pub fn update_device(&mut self, wgpu_renderer: &mut impl WgpuRendererInterface) {
        // match self.test_kind {
        //     ExampleTestsKind::TestButton => self.test_button.update_device(wgpu_renderer),
        // }   

        self.widget_renderer.update(wgpu_renderer);

    }

    pub fn resize(&mut self, abs_x: u32, abs_y: u32, size: Size) {
        self.window_width = size.width;
        self.window_height = size.height;

        match self.test_kind {
            ExampleTestsKind::TestButton => self.test_button.resize(&mut self.widget_renderer, abs_x, abs_y, size),
        }   

    }

    pub fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        // match self.test_kind {
        //     ExampleTestsKind::TestButton => self.test_button.draw(render_pass),
        // }   

        self.widget_renderer.draw(render_pass);

    }
    
    pub fn size(&mut self) -> Size {
        match self.test_kind {
            ExampleTestsKind::TestButton => self.test_button.size(),
        }   

    }
}

#[derive(Copy, Clone, Debug)]
pub enum TestButtonMessage {
    HelloWorldButtonPressed,
    HelloWorldButtonReleased,
}

struct TestButton {
    button: Button<TestButtonMessage>,
}

impl TestButton {
    fn new<'a>(
        widget_factory: &'a mut dyn WidgetFactory<TestButtonMessage>,
    ) -> Self {
        let text = "Hello World!";
        let button = widget_factory.button(text, 32, TestButtonMessage::HelloWorldButtonReleased);
        // let button = Button::new(widget_factory, text, 32).on_released(TestButtonMessage::HelloWorldButtonReleased);

        Self { button }
    }

    fn mouse_event(&mut self, mouse_event: &mouse_event::MouseEvent, model: &mut dyn FnMut(TestButtonMessage)) -> bool {
        self.button.mouse_event(mouse_event, model)
    }

    // fn update_device(&mut self, wgpu_renderer: &mut impl WgpuRendererInterface) {
    //     self.button.update_device(wgpu_renderer);
    // }

    fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, size: Size) {
        // self.button.resize(abs_x, abs_y, size);

        self.button.resize(widget_renderer, 60, 80, size);
    }

    // fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
    //     self.button.draw(render_pass);
    // }
    
    fn size(&mut self) -> Size {
        self.button.size()
    }
}

