use wgpu_gui::{core::{gui_functions::GuiElement, mouse_event, size::Size}, widget::{button::Button, widget_renderer::WidgetRenderer}};
use wgpu_renderer::renderer::WgpuRendererInterface;



enum ExampleTestsKind {
    TestButton
}

pub struct ExampleTests {
    test_kind: ExampleTestsKind,
    // widget_renderer_storage: WgpuWidgetRendererStorage,

    test_button: TestButton,

    window_width: u32,
    window_height: u32,
}

impl ExampleTests {
    pub fn new(
        renderer: &mut dyn WidgetRenderer,
    ) -> Self {
        // let mut widget_renderer_storage: WgpuWidgetRendererStorage = WgpuWidgetRendererStorage::new();
        // let mut widget_factory = WgpuWidgetRenderer{
        //     storage: &mut widget_renderer_storage,
        //     font,
        //     wgpu_renderer,
        //     texture_bind_group_layout,
        // };

        let test_button = TestButton::new(renderer);
        
        let test_kind = ExampleTestsKind::TestButton;

        Self { 
            test_kind,
            // widget_renderer_storage,

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

        // self.widget_renderer.update(wgpu_renderer);

    }

    pub fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, size: Size) {
        self.window_width = size.width;
        self.window_height = size.height;

        match self.test_kind {
            ExampleTestsKind::TestButton => self.test_button.resize(widget_renderer, abs_x, abs_y, size),
        }   

    }

    // pub fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
    //     // match self.test_kind {
    //     //     ExampleTestsKind::TestButton => self.test_button.draw(render_pass),
    //     // }   

    //     self.widget_renderer_storage.draw(render_pass);

    // }
    
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
        renderer: &'a mut dyn WidgetRenderer,
    ) -> Self {
        let text = "Hello World!";
        let button = Button::new(renderer, text, 32).on_released(TestButtonMessage::HelloWorldButtonReleased);

        Self { button }
    }

    fn mouse_event(&mut self, mouse_event: &mouse_event::MouseEvent, model: &mut dyn FnMut(TestButtonMessage)) -> bool {
        self.button.mouse_event(mouse_event, model)
    }

    fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, size: Size) {
        self.button.resize(widget_renderer, 60, 80, size);
    }

    
    fn size(&mut self) -> Size {
        self.button.size()
    }
}

