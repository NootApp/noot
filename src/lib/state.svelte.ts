export const appState = $state({
  title: "Noot",
  version: "unknown",
  arch: "unknown",
  startTime: new Date()
});
export const themeState = $state({});
export const workspace: WorkspaceState = $state({
  name: "Default Workspace",
  path: "N/A",
  lastChange: new Date(),
  configuration: {
    plugins: [],
    flavor: 0,
    rpc: {
      enable: true,
      showFileInStatus: true,
      showTimeInStatus: true,
    }
  },
  editors: [
    {
      file: 'README.md',
      name: 'README',
      opened: new Date(),
      changed: new Date(),
      hasPendingChanges: false,
      content: ""
    }
  ]

})


export enum MarkdownFlavor {
  Default = 0,
  Obsidian = 1,
  GitHub = 2
}

export interface WorkspaceState {
  name: string,
  path: string,
  lastChange: Date,
  configuration: {
    plugins: string[],
    flavor: MarkdownFlavor,
    rpc: {
      enable: boolean,
      showFileInStatus: boolean,
      showTimeInStatus: boolean
    }
  },
  editors: Editor[]
}

export interface Editor {
  file: string,
  name: string,
  opened: Date,
  changed: Date,
  hasPendingChanges: boolean,
  content: string
}
