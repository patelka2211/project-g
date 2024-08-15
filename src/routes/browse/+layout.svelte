<div class="center w-screen">
  <slot />
</div>

<div class="bottom"></div>

<style>
  .center {
    min-height: calc(100dvh - 2 * 28px);
  }

  .bottom {
    height: 28px;
    width: 100vw;
  }
</style>
