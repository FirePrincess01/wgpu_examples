
use wgpu_gui::core::gui_functions::GuiElementSubView;
use wgpu_gui::core::layout::{Alignment, Layout};
use wgpu_gui::widget::widget_renderer::WidgetRenderer;
use wgpu_gui::widget::label::Label;

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

    pub fn _update(&mut self, counter1: &counter::Counter, counter2: &counter::Counter) {
        self.text.set(String::from("lalalallalal"));
        self.sub_view1._update(counter1._value());
        self.sub_view2._update(counter2._value());
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


