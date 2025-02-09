use wgpu_gui::{core::{gui_functions::{GuiElement, GuiElementSubView, GuiElementVisitor}, layout::{Alignment, Layout}, mouse_event, size::Size, wgpu_gui::{GuiElementContainer, LayoutElements, WgpuGui}}, widget::{button::Button, label::Label, widget_renderer::WidgetRenderer}};
use wgpu_renderer::renderer::WgpuRendererInterface;



enum ExampleTestsKind {
    TestButton,
    TestLayout,
}

pub struct ExampleTests {
    test_kind: ExampleTestsKind,
    // widget_renderer_storage: WgpuWidgetRendererStorage,

    test_button: TestButton,
    test_layout: TestLayout,

    window_width: u32,
    window_height: u32,
}

impl ExampleTests {
    pub fn new(
        renderer: &mut dyn WidgetRenderer,
    ) -> Self {
        let test_button = TestButton::new(renderer);
        let test_layout = TestLayout::new(renderer);
        
        let test_kind = ExampleTestsKind::TestLayout;

        Self { 
            test_kind,

            test_button,
            test_layout,

            window_width: 0,
            window_height: 0,
        }
    }

    pub fn mouse_event(&mut self, mouse_event: &mouse_event::MouseEvent, model: &mut dyn FnMut(TestButtonMessage)) -> bool {
        
        let mut mouse_event_converted = mouse_event.clone();
        mouse_event_converted.y = if self.window_height >= mouse_event_converted.y {self.window_height - mouse_event_converted.y} else {0}; 
        
        match self.test_kind {
            ExampleTestsKind::TestButton => self.test_button.mouse_event(&mouse_event_converted, model),
            ExampleTestsKind::TestLayout => self.test_layout.mouse_event(&mouse_event_converted, model),
        }   
    }

    pub fn update_device(&mut self, widget_renderer: &mut dyn WidgetRenderer) {
        match self.test_kind {
            ExampleTestsKind::TestButton => {},
            ExampleTestsKind::TestLayout => self.test_layout.update(widget_renderer),
        }   

        // self.widget_renderer.update(wgpu_renderer);

    }

    pub fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, size: Size) {
        self.window_width = size.width;
        self.window_height = size.height;

        match self.test_kind {
            ExampleTestsKind::TestButton => self.test_button.resize(widget_renderer, abs_x, abs_y, size),
            ExampleTestsKind::TestLayout => self.test_layout.resize(widget_renderer, abs_x, abs_y, size),
        }   

    }
    
    pub fn size(&mut self) -> Size {
        match self.test_kind {
            ExampleTestsKind::TestButton => self.test_button.size(),
            ExampleTestsKind::TestLayout => self.test_layout.size(),
        }   

    }
}

#[derive(Copy, Clone, Debug)]
pub enum TestButtonMessage {
    HelloWorldButtonReleased0,
    HelloWorldButtonReleased1,
}

struct TestButton {
    button: Button<TestButtonMessage>,
}

impl TestButton {
    fn new<'a>(
        renderer: &'a mut dyn WidgetRenderer,
    ) -> Self {
        let text = "Hello World!";
        let button = Button::new(renderer, text, 32).on_released(TestButtonMessage::HelloWorldButtonReleased0);

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


pub struct TestLayout {
    button0: Button<TestButtonMessage>,
    counter: Label,
    button1: Button<TestButtonMessage>,
    layout: Layout,

    val: u32,
}

impl TestLayout {
    pub fn new<'a>(
        renderer: &'a mut dyn WidgetRenderer,
    ) -> Self {
        let button0 = Button::new(renderer, "Button 0", 32)
            .on_released(TestButtonMessage::HelloWorldButtonReleased0);

        let counter = Label::new(renderer, "000", 32);

        let button1 = Button::new(renderer, "Button 1", 32)
            .on_released(TestButtonMessage::HelloWorldButtonReleased1);

        let layout = Layout::new()
                .align(Alignment::RightBottom)
                .vertical_layout();

        Self {
            button0,
            counter,
            button1,
            layout,

            val: 10
        }
    }
}
 
impl GuiElementSubView for TestLayout {
    type TMessage = TestButtonMessage;
    type TSubMessage = TestButtonMessage;
    
    fn visit_elements(&mut self, visitor: &mut dyn GuiElementVisitor<TestButtonMessage>) {
        
        let mut elements: [&mut dyn GuiElement<_>; 3] = [
            &mut self.button0,
            &mut self.counter,
            &mut self.button1,
        ];

        visitor.visit(&mut self.layout, &mut elements);
    }
    
    fn get_event_conversion_function(&self) -> fn(Self::TSubMessage) -> Self::TMessage {
        let on_changed = |message: Self::TMessage| -> Self::TMessage { 
            println!("inner function: {:?}", message);
            // println!("{}", self.val);
            
            message 
        };

        on_changed
    }

}