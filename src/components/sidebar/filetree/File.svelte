<script lang="ts">


let {entry} = $props();
</script>

<span class="file">{entry.name}</span>

<style>
  .file {
    padding: 5px 10px;
    width: inherit;
    display: flex;
    flex-direction: row;
    white-space: nowrap;
    overflow-x: hidden;
    background: blue;
  }
</style>
