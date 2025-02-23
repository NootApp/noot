<script lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';

// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;

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
</script>

<div data-tauri-drag-region class="titlebar">
  <div class="titlebar-button" on:click={minimize} id="titlebar-minimize">
    <img
      src="/icons/mdi--window-minimize.svg"
      alt="minimize"
    />
  </div>
  <div class="titlebar-button" on:click={maximize} id="titlebar-maximize">
    <img
      src="/icons/mdi--window-maximize.svg"
      alt="maximize"
    />
  </div>
  <div class="titlebar-button" on:click={close} id="titlebar-close">
    <img src="/icons/mdi--close.svg" alt="close" />
  </div>
</div>

<style>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
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
