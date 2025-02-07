


use wgpu_gui::{core::{gui_functions::GuiElementSubView, layout::{Alignment, Layout}, wgpu_gui::{LayoutElements, WgpuGui}}, widget::{button::Button, label::Label, widget_renderer::WidgetRenderer}};

use crate::counter::{CounterMessage, Message};


pub struct CounterGuiSubView {
    button_increment: Button<CounterMessage>,
    text: Label,
    button_decrement: Button<CounterMessage>,
    layout: Layout,
    on_changed: fn(CounterMessage) -> Message,
}

impl CounterGuiSubView {
    pub fn new(renderer: &mut dyn WidgetRenderer, on_changed: fn(CounterMessage) -> Message) -> Self 
    {
        // let button_increment = widget_factory.button("increment", 32, CounterMessage::IncrementPressed);
        let button_increment = Button::new(renderer, "increment", 32).on_released(CounterMessage::IncrementPressed);
        let text = Label::new(renderer, "Hello World 2!", 32);
        // let button_decrement = widget_factory.button("decrement", 32, CounterMessage::DecrementPressed);
        let button_decrement = Button::new(renderer, "decrement", 32).on_released(CounterMessage::DecrementPressed);
        let layout = Layout::new().align(Alignment::Center).horizontal_layout();

        Self {
            button_increment,
            text,
            button_decrement,
            layout,
            on_changed,
        }
    }

    pub fn update(&mut self, val: i32) {
        self.text.set("blableblibloblu");
    }


}

impl GuiElementSubView for CounterGuiSubView {
    type TMessage = Message;
    type TSubMessage = CounterMessage;
    
    fn get_elements(&mut self, ui: &mut WgpuGui<Self::TSubMessage>) {
        ui.layout(&mut self.layout, &mut |elements: &mut LayoutElements<Self::TSubMessage>| {
            elements.add(&mut self.button_increment);
            elements.add(&mut self.text);
            elements.add(&mut self.button_decrement);
        });
    }
        
    fn get_event_conversion_function(&self) -> fn(Self::TSubMessage) -> Self::TMessage {
        self.on_changed
    }    
}

