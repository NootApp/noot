<script lang="ts">
import {workspace, wsmanifest} from "$lib/state.svelte";
import { invoke } from "@tauri-apps/api/core";
import File from "./sidebar/filetree/File.svelte";
import Symlink from "./sidebar/filetree/Symlink.svelte";
import Folder from "./sidebar/filetree/Folder.svelte";

interface TreeFile {
  File: string,
}

interface TreeSymLink {
  Symlink: string
};

type FileTree = (Folder|File|SymLink)[]

interface TreeFolder {
  Folder: NestedFolder
}

interface NestedFolder {
  node_count: number,
  parent: string,
  name: string,
  children: FileTree
}

let wd = $state("/unknown");
let tree: DirEntry[] = $state([
  {
    kind: 1,
    name: "TestFile.md",
    path: "./test.md",
  }
]);

interface DirEntry {
  kind: number,
  name: string,
  path: string,
  children?: DirEntry[]
}

function standardiseTree(folder: TreeFolder): DirEntry[] {
  let filetree = folder.Folder;
  let entries = [];
  for(let entry of filetree.children) {
    if(entry.File) {
      entries.push({
        kind: 1,
        name: entry.File,
        path: `${filetree.parent}/${filetree.name}/${entry.File}`
      })
    } else if(entry.Symlink) {
      entries.push({
        kind: 2,
        name: entry.Symlink,
        path: `${filetree.parent}/${filetree.name}/${entry.Symlink}`
      })
    } else {
      entries.push({
        kind: 3,
        name: entry.Folder.name,
        path: `${filetree.parent}/${filetree.name}/${entry.Folder.name}`,
        children: standardiseTree(entry as TreeFolder)
      })
    }
  }

  return entries;
}

$inspect(tree);

window.addEventListener("workspace-change", async() => {
  let deproxied = $state.snapshot(wsmanifest);

  console.debug("Workspace updated - reloading");
  console.debug("Workspace Manifest:",deproxied);
  if(!deproxied["disk-path"]) return;
  wd = deproxied["disk-path"];
  let [rawTree,error] = Array.from(await invoke("list_working_directory", {wd}) as any);

  if(error !== null) return console.error("Failed to update working directory - ", error);
  tree = standardiseTree({Folder:rawTree} as any);

})
</script>

<div class="sidebar-container" style="width: 15rem;height:inherit;">
  <h2>Files</h2>
  <div class="vertical">
  {#each tree as entry}
    {#if entry.kind === 1}
      <File entry={entry} />
    {:else if entry.kind === 2}
      <Symlink entry={entry} />
    {:else if entry.kind === 3}
      <Folder tree={entry} />
    {/if}
  {/each}
  </div>
</div>

<style>
.sidebar-container {
  height: calc(100vh - 45px) !important;
  border-right: 1px solid pink;
  margin: 0;
  padding: 0;
}

.vertical {
  display: flex;
  flex-direction: column;
  margin-right: 10px;
  height: calc(100vh - 85px) !important;
  background-color: purple;
}
</style>
