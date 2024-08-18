<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { getRemoteOrigin, isItRepository } from "$lib/onboarding";
  import { onMount } from "svelte";

  onMount(async () => {
    const repoPath = $page.url.searchParams.get("repo");

    if (repoPath) {
      const isRepository = await isItRepository(repoPath);

      if (isRepository === true) {
        const remote = await getRemoteOrigin(repoPath);

        if (remote === undefined) {
          // cannot find out remote origin
          console.log("cannot find out remote origin");
        } else {
          const { fetch, push } = remote;

          if (fetch === undefined && push !== undefined) {
            // only push url available
            console.log("only push url available");
          } else if (fetch !== undefined && push === undefined) {
            // only fetch url available
            console.log("only fetch url available");
          } else if (fetch === undefined && push === undefined) {
            // remote available but not origin
            console.log("remote available but not origin");
          } else {
            goto(`/browse?repo=${repoPath}`);
          }
        }
      } else {
        // not a repo
        console.log("not a repo");
        return;
      }
    } else {
      // cannot find repo
      console.log("cannot find repo");
    }
  });
</script>

onboarding
<br />
<a href="/">home</a>
