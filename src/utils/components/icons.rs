use std::sync::Arc;
use iced::widget::text;
use iced::window::Id;
use material_icons::icon_to_char;
use crate::runtime::Element;
use crate::utils::components::{Component, ComponentMessage, ComponentRegistration, COMPONENT_TRACKER};

pub struct Icon {
    icon: char,
    registration: Arc<ComponentRegistration>
}

impl Icon {
    pub fn new(icon: char, wid: Id) -> Icon {
        let registration = COMPONENT_TRACKER.lock().unwrap().register(wid);
        Self {
            icon,
            registration
        }
    }

    pub fn from_material_icon(icon: material_icons::Icon, wid: Id) -> Icon {
        let char = icon_to_char(icon);
        Self::new(char, wid)
    }

    pub fn view(&self) -> Element {
        text(self.icon.to_string())
            .into()
    }
}

impl Component for Icon {
    fn source(&self) -> Id {
        self.registration.window_id
    }

    fn view(&self) -> Element {
        self.view()
    }

    fn update(&mut self, _message: ComponentMessage) {}
}