<script lang="ts">

import {onMount, onDestroy} from "svelte"
import WidgetContainer from './WidgetContainer.svelte'


let timeString = $state(new Date().toLocaleTimeString());

let timer: number | null = null;
onMount(() => {
  if(timer !== null) window.clearInterval(timer);

  timer = window.setInterval(() => {
    timeString = new Date().toLocaleTimeString();
  }, 1000);
})

onDestroy(() => {
  if(timer !== null) window.clearInterval(timer);
})
</script>

<WidgetContainer value={timeString} />
