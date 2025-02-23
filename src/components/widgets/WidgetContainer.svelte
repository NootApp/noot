<script lang="ts">
interface WidgetContainerProps {
  value: string
}

let {value}: WidgetContainerProps = $props();
</script>

<div class="widget-container">
  <span class="widget-value">
    {value}
  </span>
</div>

<style>
  .widget-container {
    height: 30px;
    border: none;
    padding: 0;
  }

  .widget-value {
    margin: 5px 10px;
    height: 30px;
    vertical-align: middle;
  }
</style>
