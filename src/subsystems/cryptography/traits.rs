use crate::filesystem::workspace::manager::WorkspaceResult;

pub trait Secure {
    fn store<T>(&mut self, key: &str) -> WorkspaceResult<T>;
    fn load<T>(key: &str) -> WorkspaceResult<T>;
}
