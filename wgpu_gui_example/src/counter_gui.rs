
use wgpu_gui::{core::{gui::Gui, gui_element::GuiElement, gui_functions::{GuiFunctions, GuiMessageFunctions}, gui_message::GuiMessage, layout::{Alignment, Layout}, mouse_event::MouseEvent}, widget::{button::Button, text::Text}};

use crate::counter::{self, Message};

pub struct CounterGuiSubView {
    text: Text,
    layout: Layout,
}

impl CounterGuiSubView {
    pub fn new() -> Self 
    {
        let text = Text::from_space(5).size(50);
        let layout = Layout::new().align(Alignment::Center).horizontal_layout();

        Self {
            text,
            layout,
        }
    }
    pub fn update(&mut self, val: i32) {
        self.text.value(val);
    }
}

impl Gui for CounterGuiSubView {
    type TMessage = Message;

    fn layout(&mut self, f: &mut dyn FnMut(&mut Layout, &mut [&mut (dyn GuiFunctions<Self::TMessage>)])) {
        f(&mut self.layout, &mut [
            &mut self.text, 
        ]);
    }
}

pub struct CounterGui {
    button_increment: Button<Message>,
    text: Text,
    sub_view: CounterGuiSubView,
    button_decrement: Button<Message>,
    layout: Layout,
}

impl CounterGui {
    pub fn new() -> Self 
    {
        let button_increment = Button::new("increment").on_released(Message::IncrementPressed);
        let text = Text::from_space(5).size(50);
        let sub_view = CounterGuiSubView::new();
        let button_decrement = Button::new("decrement").on_released(Message::DecrementPressed);
        let layout = Layout::new().align(Alignment::Center).horizontal_layout();

        Self {
            button_increment,
            text,
            sub_view,
            button_decrement,
            layout,
        }
    }

    pub fn mouse_event_update(&mut self, mouse_event: &MouseEvent, counter: &mut counter::Counter) -> bool {
        self.mouse_event(mouse_event, &mut [counter])
    }

    pub fn update(&mut self, counter: &counter::Counter) {
        self.text.value(counter.value());
        self.sub_view.update(counter.value());
    }
}

impl Gui for CounterGui {
    type TMessage = Message;

    fn layout(&mut self, f: &mut dyn FnMut(&mut Layout, &mut [&mut (dyn GuiFunctions<Self::TMessage>)])) {
        f(&mut self.layout, &mut [
            &mut self.button_increment, 
            &mut self.text, 
            &mut self.sub_view, 
            &mut self.button_decrement
        ]);
    }
}

