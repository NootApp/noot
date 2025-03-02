use git_version::git_version;
pub(crate) const VERSION: &str = git_version!(prefix = "git:", cargo_prefix = "cargo:", fallback = "unknown");