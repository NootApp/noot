bitflags!(
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PluginScopes: u64 {
        /// Allows a plugin to see a list of the files in a workspace
        /// Scope name: `workspace.files.view`
        const SEE_WORKSPACE_FILES    = 1;

        /// Allows a plugin to list the contents of a file in the workspace
        /// Scope name: `workspace.files.modify`
        const MODIFY_WORKSPACE_FILES = 2;

        /// Collector for all workspace file permissions
        const WORKSPACE_FILES = 3;

        /// Allows a plugin to see the keybinds currently configured
        /// 
        const SEE_KEYBINDS           = 4;

        /// Allows a plugin to add, remove, or modify keybinds
        /// Scope name: `keybinds.modify`
        const MODIFY_KEYBINDS        = 8;
    }
);
