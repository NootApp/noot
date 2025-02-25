<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import TitleBar from "../components/TitleBar.svelte";
import { onMount } from "svelte";
import { listen } from "@tauri-apps/api/event";
import { workspace } from "$lib/state.svelte";

// Intermediate type to log event notifications to the console (used for ping-ponging events into the logfile)
type EventNotification = {
  name: string,
  ctx?: string,
  ok: boolean
}

let { children } = $props();

listen<EventNotification>('event-stream', (event) => {
  if(!event.payload.ctx && event.payload.ok) return console.debug('Event triggered: ', event.payload.name, event.payload.ctx);
  if(!event.payload.ctx && !event.payload.ok) return console.error('Event failed: ', event.payload.name, event.payload.ctx);
  if(event.payload.ok) return console.debug('Event triggered: ', event.payload.name, event.payload.ctx);
  console.error('Event failed: ', event.payload.name, event.payload.ctx);
})

onMount(async () => {
  console.debug("Event: mount - Starting workspace load");
  const config = await invoke("get_app_config");
  console.log(JSON.stringify(config))
  if(config.rpc && workspace.configuration.rpc.enable) {
    console.log("Enabling rpc");
    await invoke("start_rich_presence");

    console.log("Setting status");
    console.debug(workspace);
    await invoke("set_rich_presence_activity", {ws: JSON.stringify(workspace)});
  }
})

</script>

<main class="container">
  <TitleBar widgets={["clock"]} />
  {#if children}
    {@render children()}
  {:else}
    <p>fallback content</p>
  {/if}
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 5px;
}


@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}

</style>
