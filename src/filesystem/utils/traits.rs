use crate::events::types::Message;
use iced::{Task, exit};
use serde_derive::{Deserialize, Serialize};

/// A trait which ensures that all
pub trait Configuration {
    fn validate(&self, prefix: &str) -> Vec<ValidationError>;

    fn repair(&mut self);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ValidationError {
    field: String,
    message: String,
    recoverable: bool,
}

impl ValidationError {
    pub fn new(field: &str, message: &str, recoverable: bool) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
            recoverable,
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} is invalid. {}", self.field, self.message)
    }
}

pub fn list_validation_results(
    results: Vec<ValidationError>,
) -> (Task<Message>, bool) {
    let mut recoverable = true;
    for result in results {
        if result.recoverable {
            warn!("{} - {}", result.field, result.message);
        } else {
            error!("{} - {}", result.field, result.message);
            recoverable = false;
        }
    }

    if !recoverable {
        error!(
            "An irrecoverable error was encountered, and the program must now exit"
        );
        return (exit(), true);
    }

    (Task::none(), false)
}
