<script lang="ts">
  import { page } from "$app/stores";

  let repoPath = $page.url.searchParams.get("repo");
</script>

{#if repoPath === null}
  no repo provided. :(
{:else}
  browsing: {repoPath}
{/if}

<br />
<a href="/">home</a>
