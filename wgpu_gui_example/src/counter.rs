use wgpu_gui::core::gui_message::GuiMessage;


pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            value: 0,
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl GuiMessage<Message> for Counter {
    fn message(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}
