<script lang="ts">
    import { page } from "$app/stores";
    import { invoke } from "@tauri-apps/api/tauri";

    let repoPath = $page.url.searchParams.get("repo");

    interface Branch {
        name: string;
        points_at: string;
    }

    let branches: Array<Branch> | null = null;

    function refreshBranches() {
        if (typeof repoPath === "string") {
            invoke<Array<Branch> | null>("local_branches", {
                repoPath,
            }).then((value) => {
                // console.log(value);
                branches = value;
            });
        }
    }

    // $: {
    //   refreshBranches();
    // }
</script>

{#if repoPath === null}
    no repo provided. :(
{:else}
    <!-- browsing: {repoPath} -->

    {#if branches !== null}
        {branches}
    {:else}
        no branches
    {/if}
{/if}

<button on:click={refreshBranches}>refresh</button>

<br />
<a href="/">home</a>
