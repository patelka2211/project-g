## 1. Creation of Branches

- **Create a new branch**:
  - `git branch <branch-name>`
- **Create a new branch from a specific commit**:
  - `git branch <branch-name> <commit-hash | branch-name>`
- **Create a new branch and switch to it**:
  - `git checkout -b <new-branch-name>`

## 2. Maintenance of Branches

### Switching and Checking Out Branches

- **Switch to an existing branch**:
  - `git checkout <branch-name>`
  <!-- - **Check out a branch without switching**:
  - `git checkout --detach <branch-name>` -->

### Committing Changes

- **Commit changes on the current branch**:
  - `git commit -m "message"`
  <!-- - **Amend the last commit on the current branch**:
  - `git commit --amend` -->

### Merging and Rebasing

- **Merge another branch into the current branch**:
  - `git merge <branch-name>`
  <!-- - **Merge with a specific commit**:
  - `git merge <commit-hash>` -->
- **Abort a merge**:
  - `git merge --abort`
- **Rebase the current branch onto another branch**:
  - `git rebase <branch-name>`
  <!-- - **Continue after resolving rebase conflicts**:
  - `git rebase --continue` -->
  <!-- - **Skip a commit during rebase**:
  - `git rebase --skip` -->
- **Abort a rebase**:
  - `git rebase --abort`

<!-- ### Stashing Changes

- **Stash changes on the current branch**:
  - `git stash`
- **Apply stashed changes to the current branch**:
  - `git stash apply`
- **Pop stashed changes**:
  - `git stash pop` -->

<!-- ### Tagging

- **Create a tag on the current branch**:
  - `git tag <tag-name>` -->

<!-- ### Comparing and Viewing Changes

- **Compare changes between branches**:
  - `git diff <branch1>..<branch2>`
- **Show commit history for a branch**:
  - `git log <branch-name>`
- **Show changes made in a specific commit**:
  - `git show <commit-hash>` -->

### Cherry-Picking and Reverting

- **Cherry-pick a commit from another branch**:
  - `git cherry-pick <commit-hash>`
- **Revert a commit on the current branch**:
  - `git revert <commit-hash>`

### Remote "origin" Specific Actions

- **Push a new local branch to remote "origin"**:
  - `git push -u origin <branch-name>`
- **Push changes to a remote branch**:
  - `git push origin <branch-name>`
- **Fetch all branches from remote "origin"**:
  - `git fetch origin`
- **Fetch specific branche from remote "origin"**:
  - `git fetch origin <branch-name>`
- **Pull changes from a remote branch**:
  - `git pull origin <branch-name>`

## 3. Destruction of Branches

### Deleting Branches

- **Delete a local branch**:
  - `git branch -d <branch-name>` (safe delete)
- **Force delete a local branch**:
  - `git branch -D <branch-name>`
- **Delete a remote branch**:
  - `git push origin --delete <branch-name>`
