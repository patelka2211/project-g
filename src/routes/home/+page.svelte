<script lang="ts">
  import SavedRepos from "./saved-repos.svelte";
</script>

<div class="min-h-full flex flex-col items-center justify-around">
  <div class="flex flex-col items-center">
    <h1 class="font-semibold text-3xl mb-6">Welcome to Project G</h1>
    <SavedRepos />
  </div>
</div>

<style lang="scss">
  * {
    &:hover {
      cursor: default;
    }
  }
</style>
