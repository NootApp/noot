use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, SendError, Sender};
use iced::window::Id;
use crate::runtime::Element;
use crate::runtime::messaging::Message;
use crate::utils::components::buttons::ButtonMessage;

pub mod buttons;
pub mod icons;

lazy_static!(
    pub static ref COMPONENT_TRACKER: Arc<Mutex<ComponentRegistry>> = Arc::new(Mutex::new(ComponentRegistry::new()));
);

#[derive(Debug, Clone)]
pub struct ComponentRegistry {
    registry: BTreeMap<String, Arc<ComponentRegistration>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self { registry: BTreeMap::new() }
    }

    pub fn register(&mut self, id: Id) -> Arc<ComponentRegistration> {
        let registration = ComponentRegistration::new(id);

        if self.registry.contains_key(&registration.component_id) {
            panic!("Duplicate component ids in the registry");
        }

        let reg_arc = Arc::new(registration);

        self.registry.insert(reg_arc.component_id.clone(), reg_arc.clone());

        reg_arc
    }

    pub fn deregister(&mut self, id: String) -> Option<Arc<ComponentRegistration>> {
        self.registry.remove(&id)
    }

    pub fn get(&self, id: &String) -> Option<Arc<ComponentRegistration>> {
        self.registry.get(id).cloned()
    }

    pub fn get_mut(&mut self, id: &String) -> Option<Arc<ComponentRegistration>> {
        self.registry.get_mut(id).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct ComponentRegistration {
    pub component_id: String,
    pub window_id: Id,
}

impl ComponentRegistration {
    pub fn new(window_id: Id) -> Self {
        Self {
            component_id: nanoid!(),
            window_id,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ComponentMessageKind {
    Button(ButtonMessage),
}

#[derive(Clone, Debug)]
pub struct ComponentMessage {
    pub kind: ComponentMessageKind,
    pub window_id: Id,
}

impl Into<Message> for ComponentMessage {
    fn into(self) -> Message {
        Message::component(self)
    }
}

impl ComponentMessage {
    pub fn new(kind: ComponentMessageKind, window_id: Id) -> Self {
        Self { kind, window_id }
    }

    pub fn button(button_message: ButtonMessage) -> Self {
        Self::new(ComponentMessageKind::Button(button_message.clone()), button_message.window_id)
    }
}

pub trait Component {
    fn source(&self) -> Id;
    fn view(&self) -> Element;
    fn update(&mut self, message: ComponentMessage);
}