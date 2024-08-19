<script>
  import { ModeWatcher } from "mode-watcher";
  import "../app.css";
</script>

<ModeWatcher />

<div data-tauri-drag-region class="border-b border-accent"></div>

<slot></slot>

<style>
  div {
    height: 28px;
    width: 100vw;
  }
</style>
