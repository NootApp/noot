bitflags!(
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PluginScopes: u64 {
        const THEME = 1;

        /// Allows a plugin to see a list of the files in a workspace
        /// Scope name: `workspace.files.view`
        const SEE_WORKSPACE_FILES    = 2;

        /// Allows a plugin to list the contents of a file in the workspace
        /// Scope name: `workspace.files.modify`
        const MODIFY_WORKSPACE_FILES = 3;

        /// Collector for all workspace file permissions
        const WORKSPACE_FILES = 4;

        /// Allows a plugin to see the keybinds currently configured
        /// Scope name: `keybinds.view`
        const SEE_KEYBINDS           = 5;

        /// Allows a plugin to add, remove, or modify keybinds
        /// Scope name: `keybinds.modify`
        const MODIFY_KEYBINDS        = 6;








        /// Allows a plugin to access methods which may allow it to
        /// escape from sandboxing and affect the host machine
        ///
        /// > WARNING: Any plugin using this flag will be required to be enabled
        /// by the user for __every session__. Additionally, this plugin will be
        /// required to maintain a source code repository which is open to public
        /// scrutiny. All plugins using this scope will be ineligible for sandbox
        /// bypass unless they comply with all the terms set, to protect our users,
        /// and we reserve the right to immediately, and without reason, rescind
        /// this ability from any plugin, at any time, without notice to the maintainers.
        ///
        /// This scope is only available for non-enterprise builds of Noot.
        const AVOID_SANDBOXES        = 9_223_372_036_854_775_808;
    }
);

