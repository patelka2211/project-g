<div class="center w-screen flex flex-col items-center">
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
