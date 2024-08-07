<script>
  import { page } from "$app/stores";

  let repo = $page.url.searchParams.get("repo-path");
</script>

{#if repo === null}
  No repo provided!
{:else}
  checking repo: {repo}
{/if}

<br />

<a href="/">home</a>
