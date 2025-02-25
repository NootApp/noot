<script lang="ts">
import File from "./File.svelte";
import Symlink from "./Symlink.svelte";
import Self from "./Folder.svelte"

let {tree} = $props();
</script>
<div class="folder">
  <span>{tree.name}</span>
  <div class="children">
    {#each tree.children as entry}
      {#if entry.kind === 1}
        <File entry={entry} />
      {:else if entry.kind === 2}
        <Symlink entry={entry} />
      {:else if entry.kind === 3}
        <Self tree={entry} />
      {/if}
    {/each}
  </div>
</div>

<style>
.folder {
  background-color: green;
  display: flex;
  flex-direction: column;
  height: fit-content;
  margin: 2px 0;
}

.children {
  margin: 2px 0;
  padding: 0px 5px
}
</style>
