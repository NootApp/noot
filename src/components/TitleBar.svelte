<script lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import renderWidget from "./widgets/renderer";

import {appState} from "$lib/state.svelte";

// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;



interface TitleBarProps {
  widgets: string[]
}

let {widgets}: TitleBarProps = $props();

const appWindow = getCurrentWindow();

async function minimize() {
  console.log("Minimizing");
  await appWindow.isMinimized() ? appWindow.unminimize() : appWindow.minimize();
}

async function maximize() {
  console.log("Maximizing");

  await appWindow.isMaximized() ? appWindow.unmaximize() : appWindow.maximize();
}

async function close() {
  console.log("Closing");
  await appWindow.close();
}

let title = $state("This is a test")

$effect(() => {
  title = appState.title;
  window.title = title
})

</script>

<div data-tauri-drag-region class="titlebar">
  <span class="noto-sans-600">{title}</span>
  {#if widgets!==undefined && widgets.length > 0}
    {#each widgets as widget}
      {#await renderWidget(widget) then componentWidget}
        <div style="border-left:2px solid #00000055;margin: 5px"></div>

        <svelte:component this={componentWidget} />
      {/await}
    {/each}
  {/if}
  <div class="titlebar-right">
    <div class="titlebar-button" onclick={minimize} role="button" id="titlebar-minimize">
      <img
        src="/icons/mdi--window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div class="titlebar-button" onclick={maximize} role="button" id="titlebar-maximize">
      <img
        src="/icons/mdi--window-maximize.svg"
        alt="maximize"
      />
    </div>
    <div class="titlebar-button" onclick={close} role="button" id="titlebar-close">
      <img src="/icons/mdi--close.svg" alt="close" />
    </div>
  </div>
</div>

<style>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-start;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}

.titlebar span {
  margin: auto 5px;

}

.titlebar-right {
  display: flex;
  justify-content: flex-end;
  height: 30px;
  width: fit-content;
  margin-left: auto;
  margin-right: 0px;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  user-select: none;
  -webkit-user-select: none;
}
.titlebar-button:hover {
  background: #5bbec3;
}
</style>
