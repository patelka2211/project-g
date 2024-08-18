<script>
  import { ModeWatcher } from "mode-watcher";
  import "../app.css";
</script>

<ModeWatcher />

<div data-tauri-drag-region></div>

<slot></slot>

<style>
  div {
    height: 28px;
    width: 100vw;
  }
</style>
