export let appState = $state({
  title: "Noot",
  version: "unknown",
  arch: "unknown",
  startTime: new Date()
});
export const themeState = $state({});
export const wsmanifest: WorkspaceManifest = $state({});
export const workspace: WorkspaceState = $state({});

export enum WorkspaceManifestFormatVersion {
  V001 = "V001"
}

export interface WorkspaceManifestList {
  format: WorkspaceManifestFormatVersion,
  "last-opened": String,
  workspaces: WorkspaceManifest[]
}

export interface WorkspaceManifest {
  id: string,
  "display-name": string,
  "disk-path": string,
  "last-accessed": string
}

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



export function setWorkspaceState(wss: WorkspaceState) {
  for (const [k,v] of Object.entries(wss)) {
    // @ts-ignore: This shouldn't pose an issue but typescript wants to whine regardless.
    wsmanifest[k] = v;
  }

  wsmanifest["last-accessed"] = new Date();

  globalThis.dispatchEvent(new Event("workspace-change"));
}
