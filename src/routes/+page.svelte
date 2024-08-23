<script lang="ts">
  import { goto } from "$app/navigation";
  import { isGitAvailable } from "@/integrated-backend/init";
  import { onMount } from "svelte";

  let errorMsg: string | undefined;

  onMount(async () => {
    try {
      let gitExist = await isGitAvailable();

      if (gitExist === true) {
        goto("/home");
      } else {
        errorMsg = "Git not available!";
      }
    } catch (error) {
      errorMsg = "Not able to find Git!";
      if (error instanceof Error) {
        console.log(error.message);
      }
    }
  });
</script>

{#if errorMsg !== undefined}
  <div class="w-full h-full flex flex-col items-center justify-around">
    <span>{errorMsg}</span>
  </div>
{/if}
