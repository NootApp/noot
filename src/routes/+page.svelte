<script lang="ts">
import { rebindConsole } from "$lib/console";
import { invoke } from "@tauri-apps/api/core";
import SideBar from "../components/SideBar.svelte";
import TitleBar from "../components/TitleBar.svelte";
import { configureTrayIcon } from "../lib/tray-app";
import { onMount } from "svelte";

rebindConsole()

let name = $state("");
let greetMsg = $state("");

async function greet(event: Event) {
  event.preventDefault();
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg = await invoke("greet", { name });
}
</script>

<main class="container">
  <TitleBar widgets={["clock"]} />
  <SideBar />

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
