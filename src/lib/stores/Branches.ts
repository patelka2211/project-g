import {
  getLocalBranches,
  type BranchInfo,
} from "@/integrated-backend/browse/branches";
import { writable } from "svelte/store";

export let branches = (() => {
  let branchesStore = writable<Array<BranchInfo>>([]);

  async function reload(repoPath: string) {
    try {
      let branches = (await getLocalBranches(repoPath)).sort((a, b) => {
        if (a.commitTime === b.commitTime) return a.name.localeCompare(b.name);
        else return b.commitTime - a.commitTime;
      });

      branchesStore.set(branches);
    } catch (error) {
      console.error(error);
    }
  }

  return {
    ...branchesStore,
    reload,
  };
})();
