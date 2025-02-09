use wgpu_gui::core::gui_message::GuiMessage;

#[derive(Copy, Clone)]
pub enum CounterMessage {
    IncrementPressed,
    DecrementPressed,
}

#[derive(Copy, Clone)]
pub enum Message {
    SubView1(CounterMessage),
    SubView2(CounterMessage),
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

impl GuiMessage<CounterMessage> for Counter {
    fn message(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::IncrementPressed => {
                self.value += 1;
            }
            CounterMessage::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}
