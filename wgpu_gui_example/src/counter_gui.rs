
use wgpu_gui::{core::{gui_functions::GuiElementSubView, layout::{Alignment, Layout}, wgpu_gui::{LayoutElements, WgpuGui}}, wgpu::{wgpu_widget_factory::WgpuWidgetFactory, wgpu_widget_renderer::WgpuWidgetRenderer}, widget::{text::Text, widget_factory::WidgetFactory, widget_renderer}};

use crate::{counter::{self, Message}, counter_gui_subview::CounterGuiSubView};

pub struct CounterGui {
    text: Text,
    sub_view1: CounterGuiSubView,
    sub_view2: CounterGuiSubView,
    layout: Layout,
    on_changed: fn(Message) -> Message,
}

impl CounterGui {
    pub fn new(
        font: &rusttype::Font<'static>, 
        wgpu_renderer: &mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
        texture_bind_group_layout: &wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout,

    ) -> Self 
    {
        let mut widget_renderer = WgpuWidgetRenderer::new();
        let mut widget_factory = WgpuWidgetFactory::new(font, wgpu_renderer, texture_bind_group_layout, &mut widget_renderer);

        let text = Text::from_space(5).size(50);
        let sub_view1 = CounterGuiSubView::new(&mut widget_factory, Message::SubView1);
        let sub_view2 = CounterGuiSubView::new(&mut widget_factory, Message::SubView2);
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
        self.text.value(counter1.value());
        self.sub_view1.update(counter1.value());
        self.sub_view2.update(counter2.value());
    }
}


impl GuiElementSubView for CounterGui {
    type TMessage = Message;
    type TSubMessage = Message;
    
    fn get_elements(&mut self, ui: &mut WgpuGui<Self::TSubMessage>) {
        ui.layout(&mut self.layout, &mut |elements: &mut LayoutElements<Self::TSubMessage>| {
            elements.add(&mut self.text);
            elements.add(&mut self.sub_view1);
            elements.add(&mut self.sub_view2);
        });
    }
        
    fn get_event_conversion_function(&self) -> fn(Self::TSubMessage) -> Self::TMessage {
        self.on_changed
    }    
}


