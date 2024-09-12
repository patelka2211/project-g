import { getLocalBranches } from "@/integrated-backend/browse/branches";
import type { BranchInfo } from "@/integrated-backend/browse/branches/types";
import { writable } from "svelte/store";

export let branches = (() => {
  let store = writable<Array<BranchInfo>>([]);

  async function reload(repoPath: string) {
    try {
      let branches = (await getLocalBranches(repoPath)).sort((a, b) => {
        if (a.commitTime === b.commitTime) return a.name.localeCompare(b.name);
        else return b.commitTime - a.commitTime;
      });

      store.set(branches);
    } catch (error) {
      console.error(error);
    }
  }

  return {
    ...store,
    reload,
  };
})();
