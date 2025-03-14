use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
    pub struct WorkspaceFlags: u32 {
        const NONE = 0b00000000;

        /// Whether the workspace is encrypted
        const ENCRYPTED = 0b00000001;

        /// Whether the workspace is managed using the Noot Workspace Manager Protocol
        const NWMP = 0b00000010;

        /// Plugin support, do not supply the bit if
        const ALLOW_PLUGINS = 0b00000100;

        /// Forces the workspace to synchronize in "immediate" mode
        const FORCE_CLEAN = 0b00001000;

        /// Enterprise mode - See the above entries for definitions
        const ENTERPRISE = Self::ENCRYPTED.bits()
            | Self::NWMP.bits()
            // | Self::ALLOW_PLUGINS.bits()
            | Self::FORCE_CLEAN.bits();

        const DEFAULTS = Self::ALLOW_PLUGINS.bits()
            | Self::FORCE_CLEAN.bits();
    }
}

impl Default for WorkspaceFlags {
    fn default() -> Self {
        WorkspaceFlags::DEFAULTS
    }
}

impl From<WorkspaceFlags> for u32 {
    fn from(permissions: WorkspaceFlags) -> Self {
        permissions.bits()
    }
}

impl From<u32> for WorkspaceFlags {
    fn from(value: u32) -> Self {
        WorkspaceFlags::from_bits_truncate(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::filesystem::workspace::global::flags::WorkspaceFlags;

    #[test]
    fn test_enterprise_features_match() {
        let test_target = WorkspaceFlags::ENTERPRISE;

        let test_value: u32 = 11;

        assert_eq!(test_target.bits(), test_value);
    }

    #[test]
    fn test_default_features_match() {
        let test_target = WorkspaceFlags::default();
        let test_value: u32 = 12;

        assert_eq!(test_target.bits(), test_value);
    }
}
