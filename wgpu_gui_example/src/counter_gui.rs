
use wgpu_gui::{core::{gui_functions::{GuiElement, GuiElementSubView}, layout::{Alignment, Layout}, wgpu_gui::{LayoutElements, WgpuGui}}, widget::{label::Label, widget_renderer::WidgetRenderer}};

use crate::{counter::{self, Message}, counter_gui_subview::CounterGuiSubView};

pub struct CounterGui {
    text: Label,
    sub_view1: CounterGuiSubView,
    sub_view2: CounterGuiSubView,
    layout: Layout,
    on_changed: fn(Message) -> Message,
}

impl CounterGui {
    pub fn new(
        renderer: &mut dyn WidgetRenderer,
    ) -> Self 
    {
        let text = Label::new(renderer, "Hello World!", 32);
        let sub_view1 = CounterGuiSubView::new( renderer, Message::SubView1);
        let sub_view2 = CounterGuiSubView::new(renderer, Message::SubView2);
        let layout = Layout::new().align(Alignment::Center).horizontal_layout();
        let on_changed = |message: Message| -> Message { message };

        Self {
            text,
            sub_view1,
            sub_view2,
            layout,
            on_changed,
        }
    }

    pub fn update(&mut self, counter1: &counter::Counter, counter2: &counter::Counter) {
        self.text.set(String::from("lalalallalal"));
        self.sub_view1.update(counter1.value());
        self.sub_view2.update(counter2.value());
    }
}


impl GuiElementSubView for CounterGui{
    type TMessage = Message;
    type TSubMessage = Message;
    
    fn visit_elements(&mut self, visitor: &mut dyn wgpu_gui::core::gui_functions::GuiElementVisitor<Self::TSubMessage>) {
        visitor.visit(&mut self.layout, &mut [
            &mut self.text,
            &mut self.sub_view1,
            &mut self.sub_view2,
        ]);
    }
        
    fn on_event(&mut self, event: Self::TSubMessage) -> Self::TMessage {
        (self.on_changed)(event)
    }  
}


