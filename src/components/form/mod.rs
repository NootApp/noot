pub mod text_input;

/// FormEvent is used to emit an event from within a form
#[derive(Debug, Clone)]
pub enum FormEvent {
    FormTextInputChanged(String, String),
}