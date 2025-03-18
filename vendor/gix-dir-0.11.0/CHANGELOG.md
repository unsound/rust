# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.11.0 (2024-12-22)

<csr-id-c06a57e698ee21d6c6d8a35bbb37323ceef1e143/>

### Other

 - <csr-id-c06a57e698ee21d6c6d8a35bbb37323ceef1e143/> Clarify and expand descriptions of `NonFile`s
   Discussed in:
   https://github.com/GitoxideLabs/gitoxide/pull/1730#discussion_r1894381449
   
   At least for now, they remain called `NonFile`s (and sometimes
   referred to as "non-files" in text), but more specifically defined.

### Bug Fixes (BREAKING)

 - <csr-id-d90412bdf86c70ecb5a6d0a1fefae875b3eb836b/> rename `entry::Kind::NonFile` to `entry::Kind::Untrackable`.
 - <csr-id-0d517716b81e203a131552c5163894a4fbf2863e/> Add `entry::Kind::NonFile`.
   Previously, these were misclassified as `File`, which can lead to blocking applications
   which get stuck reading pipes.
   
   Now the downstream is forced to deal with the possibility that the item at hand isn't a file,
   to do application-specific things.
 - <csr-id-f7ffb9183f918a9521a8f21d3cc0c70925d4b0f1/> assure `fifo` or non-files aren't considered files, but are pruned instead.
   That way, algorithms relying on dirwalking can still see them if they want to,
   but would have a hard time to use them (accidentally).
   
   Note that this replaces the `From` implementation with `entry::Kind::try_from_file_type()`,
   which makes this a breaking change.`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 28 calendar days.
 - 28 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.9.3, gix-object v0.46.1, gix-command v0.4.0, gix-filter v0.16.0, gix-fs v0.12.1, gix-traverse v0.43.1, gix-worktree-stream v0.18.0, gix-archive v0.18.0, gix-ref v0.49.1, gix-prompt v0.9.0, gix-url v0.28.2, gix-credentials v0.26.0, gix-diff v0.49.0, gix-dir v0.11.0, gix-revision v0.31.1, gix-merge v0.2.0, gix-pack v0.56.0, gix-odb v0.66.0, gix-shallow v0.1.0, gix-packetline v0.18.2, gix-transport v0.44.0, gix-protocol v0.47.0, gix-status v0.16.0, gix-worktree-state v0.16.0, gix v0.69.0, gitoxide-core v0.44.0, gitoxide v0.40.0, safety bump 16 crates ([`c1ba571`](https://github.com/GitoxideLabs/gitoxide/commit/c1ba5719132227410abefeb54e3032b015233e94))
    - Update changelogs prior to release ([`7ea8582`](https://github.com/GitoxideLabs/gitoxide/commit/7ea85821c6999e3e6cf50a2a009904e9c38642a4))
    - Merge pull request #1734 from EliahKagan/nonfiles ([`ad6b9b6`](https://github.com/GitoxideLabs/gitoxide/commit/ad6b9b66aa3e3561e413d04d00f6dbf832d63353))
    - Reword "non-files" in documentation comments ([`154b21f`](https://github.com/GitoxideLabs/gitoxide/commit/154b21f0e9beb0e5b6615f091f5f0716df6a3f7b))
    - Rename `entry::Kind::NonFile` to `entry::Kind::Untrackable`. ([`d90412b`](https://github.com/GitoxideLabs/gitoxide/commit/d90412bdf86c70ecb5a6d0a1fefae875b3eb836b))
    - Clarify and expand descriptions of `NonFile`s ([`c06a57e`](https://github.com/GitoxideLabs/gitoxide/commit/c06a57e698ee21d6c6d8a35bbb37323ceef1e143))
    - Merge pull request #1730 from GitoxideLabs/fix-1729 ([`6822689`](https://github.com/GitoxideLabs/gitoxide/commit/6822689fca04c15e309f9ca41d610bca9cb93e3b))
    - Add `entry::Kind::NonFile`. ([`0d51771`](https://github.com/GitoxideLabs/gitoxide/commit/0d517716b81e203a131552c5163894a4fbf2863e))
    - Merge pull request #1727 from GitoxideLabs/dirwalk-ignore-non-regulars ([`69ee6a3`](https://github.com/GitoxideLabs/gitoxide/commit/69ee6a32dd221a1aae7b8c3817f90feacf577598))
    - Assure `fifo` or non-files aren't considered files, but are pruned instead. ([`f7ffb91`](https://github.com/GitoxideLabs/gitoxide/commit/f7ffb9183f918a9521a8f21d3cc0c70925d4b0f1))
    - Add a test to show what FIFO does to the dirwalk. ([`949fe2c`](https://github.com/GitoxideLabs/gitoxide/commit/949fe2c4da0a74a0c45cfa8cf820f3ce7a808cda))
    - Refactor test-structure to match the 'new style'. ([`e6199a5`](https://github.com/GitoxideLabs/gitoxide/commit/e6199a5819d2567e9549b16020d69bfc55057dc7))
    - Merge pull request #1701 from GitoxideLabs/release ([`e8b3b41`](https://github.com/GitoxideLabs/gitoxide/commit/e8b3b41dd79b8f4567670b1f89dd8867b6134e9e))
</details>

## 0.10.0 (2024-11-24)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-glob v0.17.1, gix-command v0.3.11, gix-filter v0.15.0, gix-chunk v0.4.10, gix-commitgraph v0.25.1, gix-revwalk v0.17.0, gix-traverse v0.43.0, gix-worktree-stream v0.17.0, gix-archive v0.17.0, gix-config-value v0.14.10, gix-lock v15.0.1, gix-ref v0.49.0, gix-sec v0.10.10, gix-config v0.42.0, gix-prompt v0.8.9, gix-url v0.28.1, gix-credentials v0.25.1, gix-ignore v0.12.1, gix-bitmap v0.2.13, gix-index v0.37.0, gix-worktree v0.38.0, gix-diff v0.48.0, gix-discover v0.37.0, gix-pathspec v0.8.1, gix-dir v0.10.0, gix-mailmap v0.25.1, gix-revision v0.31.0, gix-merge v0.1.0, gix-negotiate v0.17.0, gix-pack v0.55.0, gix-odb v0.65.0, gix-packetline v0.18.1, gix-transport v0.43.1, gix-protocol v0.46.1, gix-refspec v0.27.0, gix-status v0.15.0, gix-submodule v0.16.0, gix-worktree-state v0.15.0, gix v0.68.0, gix-fsck v0.8.0, gitoxide-core v0.43.0, gitoxide v0.39.0 ([`4000197`](https://github.com/GitoxideLabs/gitoxide/commit/4000197ecc8cf1a5d79361620e4c114f86476703))
    - Release gix-date v0.9.2, gix-actor v0.33.1, gix-hash v0.15.1, gix-features v0.39.1, gix-validate v0.9.2, gix-object v0.46.0, gix-path v0.10.13, gix-quote v0.4.14, gix-attributes v0.23.1, gix-packetline-blocking v0.18.1, gix-filter v0.15.0, gix-chunk v0.4.10, gix-commitgraph v0.25.1, gix-revwalk v0.17.0, gix-traverse v0.43.0, gix-worktree-stream v0.17.0, gix-archive v0.17.0, gix-config-value v0.14.10, gix-lock v15.0.1, gix-ref v0.49.0, gix-config v0.42.0, gix-prompt v0.8.9, gix-url v0.28.1, gix-credentials v0.25.1, gix-bitmap v0.2.13, gix-index v0.37.0, gix-worktree v0.38.0, gix-diff v0.48.0, gix-discover v0.37.0, gix-pathspec v0.8.1, gix-dir v0.10.0, gix-mailmap v0.25.1, gix-revision v0.31.0, gix-merge v0.1.0, gix-negotiate v0.17.0, gix-pack v0.55.0, gix-odb v0.65.0, gix-packetline v0.18.1, gix-transport v0.43.1, gix-protocol v0.46.1, gix-refspec v0.27.0, gix-status v0.15.0, gix-submodule v0.16.0, gix-worktree-state v0.15.0, gix v0.68.0, gix-fsck v0.8.0, gitoxide-core v0.43.0, gitoxide v0.39.0, safety bump 25 crates ([`8ce4912`](https://github.com/GitoxideLabs/gitoxide/commit/8ce49129a75e21346ceedf7d5f87fa3a34b024e1))
    - Prepare changelogs prior to release ([`bc9d994`](https://github.com/GitoxideLabs/gitoxide/commit/bc9d9943e8499a76fc47a05b63ac5c684187d1ae))
    - Merge pull request #1662 from paolobarbolini/thiserror-v2 ([`7a40648`](https://github.com/GitoxideLabs/gitoxide/commit/7a406481b072728cec089d7c05364f9dbba335a2))
    - Upgrade thiserror to v2.0.0 ([`0f0e4fe`](https://github.com/GitoxideLabs/gitoxide/commit/0f0e4fe121932a8a6302cf950b3caa4c8608fb61))
    - Merge pull request #1652 from EliahKagan/run-ci/chmod ([`8e99eba`](https://github.com/GitoxideLabs/gitoxide/commit/8e99eba2a284b35b5e9bcb97e47bfbbafc3df5d1))
    - Enable gix-dir walk symlink tests even on Windows ([`d03971b`](https://github.com/GitoxideLabs/gitoxide/commit/d03971b27f5b4fffc009329edb05f8ac0baac1d8))
    - Merge pull request #1646 from EliahKagan/run-ci/portable-rm ([`35d4d9f`](https://github.com/GitoxideLabs/gitoxide/commit/35d4d9f801fa60875ebcd6102d874522be861e18))
    - Avoid `cp -v` in fixtures for portability ([`dffb694`](https://github.com/GitoxideLabs/gitoxide/commit/dffb6945508023eb87c14c79ba59bb23723193ff))
    - Merge pull request #1642 from GitoxideLabs/new-release ([`db5c9cf`](https://github.com/GitoxideLabs/gitoxide/commit/db5c9cfce93713b4b3e249cff1f8cc1ef146f470))
</details>

## 0.9.0 (2024-10-22)

<csr-id-64ff0a77062d35add1a2dd422bb61075647d1a36/>

### Other

 - <csr-id-64ff0a77062d35add1a2dd422bb61075647d1a36/> Update gitoxide repository URLs
   This updates `Byron/gitoxide` URLs to `GitoxideLabs/gitoxide` in:
   
   - Markdown documentation, except changelogs and other such files
     where such changes should not be made.
   
   - Documentation comments (in .rs files).
   
   - Manifest (.toml) files, for the value of the `repository` key.
   
   - The comments appearing at the top of a sample hook that contains
     a repository URL as an example.
   
   When making these changes, I also allowed my editor to remove
   trailing whitespace in any lines in files already being edited
   (since, in this case, there was no disadvantage to allowing this).
   
   The gitoxide repository URL changed when the repository was moved
   into the recently created GitHub organization `GitoxideLabs`, as
   detailed in #1406. Please note that, although I believe updating
   the URLs to their new canonical values is useful, this is not
   needed to fix any broken links, since `Byron/gitoxide` URLs
   redirect (and hopefully will always redirect) to the coresponding
   `GitoxideLabs/gitoxide` URLs.
   
   While this change should not break any URLs, some affected URLs
   were already broken. This updates them, but they are still broken.
   They will be fixed in a subsequent commit.
   
   This also does not update `Byron/gitoxide` URLs in test fixtures
   or test cases, nor in the `Makefile`. (It may make sense to change
   some of those too, but it is not really a documentation change.)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 60 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.9.1, gix-utils v0.1.13, gix-actor v0.33.0, gix-hash v0.15.0, gix-trace v0.1.11, gix-features v0.39.0, gix-hashtable v0.6.0, gix-validate v0.9.1, gix-object v0.45.0, gix-path v0.10.12, gix-glob v0.17.0, gix-quote v0.4.13, gix-attributes v0.23.0, gix-command v0.3.10, gix-packetline-blocking v0.18.0, gix-filter v0.14.0, gix-fs v0.12.0, gix-chunk v0.4.9, gix-commitgraph v0.25.0, gix-revwalk v0.16.0, gix-traverse v0.42.0, gix-worktree-stream v0.16.0, gix-archive v0.16.0, gix-config-value v0.14.9, gix-tempfile v15.0.0, gix-lock v15.0.0, gix-ref v0.48.0, gix-sec v0.10.9, gix-config v0.41.0, gix-prompt v0.8.8, gix-url v0.28.0, gix-credentials v0.25.0, gix-ignore v0.12.0, gix-bitmap v0.2.12, gix-index v0.36.0, gix-worktree v0.37.0, gix-diff v0.47.0, gix-discover v0.36.0, gix-pathspec v0.8.0, gix-dir v0.9.0, gix-mailmap v0.25.0, gix-merge v0.0.0, gix-negotiate v0.16.0, gix-pack v0.54.0, gix-odb v0.64.0, gix-packetline v0.18.0, gix-transport v0.43.0, gix-protocol v0.46.0, gix-revision v0.30.0, gix-refspec v0.26.0, gix-status v0.14.0, gix-submodule v0.15.0, gix-worktree-state v0.14.0, gix v0.67.0, gix-fsck v0.7.0, gitoxide-core v0.42.0, gitoxide v0.38.0, safety bump 41 crates ([`3f7e8ee`](https://github.com/GitoxideLabs/gitoxide/commit/3f7e8ee2c5107aec009eada1a05af7941da9cb4d))
    - Merge pull request #1624 from EliahKagan/update-repo-url ([`795962b`](https://github.com/GitoxideLabs/gitoxide/commit/795962b107d86f58b1f7c75006da256d19cc80ad))
    - Update gitoxide repository URLs ([`64ff0a7`](https://github.com/GitoxideLabs/gitoxide/commit/64ff0a77062d35add1a2dd422bb61075647d1a36))
    - Merge pull request #1612 from Byron/merge ([`37c1e4c`](https://github.com/GitoxideLabs/gitoxide/commit/37c1e4c919382c9d213bd5ca299ed659d63ab45d))
    - Thanks clippy ([`af03832`](https://github.com/GitoxideLabs/gitoxide/commit/af0383254422b70d53f27572c415eea2e4154447))
    - Merge pull request #1582 from Byron/gix-path-release ([`93e86f1`](https://github.com/GitoxideLabs/gitoxide/commit/93e86f12a8d0ab59ad5d885ce552d0dec9a6fba6))
    - Release gix-trace v0.1.10, gix-path v0.10.11 ([`012a754`](https://github.com/GitoxideLabs/gitoxide/commit/012a75455edebc857ff13c97c1e7603ea5ea6cdc))
    - Merge pull request #1557 from Byron/merge-base ([`649f588`](https://github.com/GitoxideLabs/gitoxide/commit/649f5882cbebadf1133fa5f310e09b4aab77217e))
    - Allow empty-docs ([`beba720`](https://github.com/GitoxideLabs/gitoxide/commit/beba7204a50a84b30e3eb81413d968920599e226))
    - Merge branch 'global-lints' ([`37ba461`](https://github.com/GitoxideLabs/gitoxide/commit/37ba4619396974ec9cc41d1e882ac5efaf3816db))
    - Workspace Clippy lint management ([`2e0ce50`](https://github.com/GitoxideLabs/gitoxide/commit/2e0ce506968c112b215ca0056bd2742e7235df48))
    - Merge pull request #1546 from nyurik/semilocons ([`f992fb7`](https://github.com/GitoxideLabs/gitoxide/commit/f992fb773b443454015bd14658cfaa2f3ac07997))
    - Add missing semicolons ([`ec69c88`](https://github.com/GitoxideLabs/gitoxide/commit/ec69c88fc119f3aa1967a7e7f5fca30e3ce97595))
</details>

## 0.8.0 (2024-08-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-actor v0.32.0, gix-object v0.44.0, gix-filter v0.13.0, gix-revwalk v0.15.0, gix-traverse v0.41.0, gix-worktree-stream v0.15.0, gix-archive v0.15.0, gix-ref v0.47.0, gix-config v0.40.0, gix-index v0.35.0, gix-worktree v0.36.0, gix-diff v0.46.0, gix-discover v0.35.0, gix-dir v0.8.0, gix-mailmap v0.24.0, gix-negotiate v0.15.0, gix-pack v0.53.0, gix-odb v0.63.0, gix-revision v0.29.0, gix-refspec v0.25.0, gix-status v0.13.0, gix-submodule v0.14.0, gix-worktree-state v0.13.0, gix v0.66.0, gix-fsck v0.6.0, gitoxide-core v0.41.0, gitoxide v0.38.0, safety bump 26 crates ([`b3ff033`](https://github.com/GitoxideLabs/gitoxide/commit/b3ff033b602f303433f0b2e4daa2dba90b619c9e))
    - Prepare changelog prior to (yet another) release ([`209b6de`](https://github.com/GitoxideLabs/gitoxide/commit/209b6de0329dbaaf61b929d32d9d54cf13fe241e))
</details>

## 0.7.0 (2024-08-22)

### Bug Fixes

 - <csr-id-31e795abf27dea8fd7dd7f59996cdd5d70398601/> Assure that worktrees in hidden directories are not deleted
 - <csr-id-6c8850b44e288548a81db24974779f3811433fc9/> assure an ignored repository clone is for deletion is recognized as repository

### New Features (BREAKING)

 - <csr-id-c9cd2d258a46ba3dc93559abea9a95405e62f930/> make it possible to consider worktrees to be 'tracked'
   That way it's possibel for them to be equivalent to submodules, which
   would never be deleted by accident due to their 'tracked' status.
   
   This works by passing repository-relative paths of worktree locations
   that are within this repository.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 28 calendar days.
 - 30 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#1464](https://github.com/GitoxideLabs/gitoxide/issues/1464), [#1469](https://github.com/GitoxideLabs/gitoxide/issues/1469), [#1470](https://github.com/GitoxideLabs/gitoxide/issues/1470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1464](https://github.com/GitoxideLabs/gitoxide/issues/1464)**
    - Make it possible to consider worktrees to be 'tracked' ([`c9cd2d2`](https://github.com/GitoxideLabs/gitoxide/commit/c9cd2d258a46ba3dc93559abea9a95405e62f930))
    - Assure an ignored repository clone is for deletion is recognized as repository ([`6c8850b`](https://github.com/GitoxideLabs/gitoxide/commit/6c8850b44e288548a81db24974779f3811433fc9))
 * **[#1469](https://github.com/GitoxideLabs/gitoxide/issues/1469)**
    - Add a test to see what happens if worktrees are hidden in ignored directories ([`2bacc45`](https://github.com/GitoxideLabs/gitoxide/commit/2bacc45f794d300656ca770c803a63c5df8b71fc))
 * **[#1470](https://github.com/GitoxideLabs/gitoxide/issues/1470)**
    - Assure that worktrees in hidden directories are not deleted ([`31e795a`](https://github.com/GitoxideLabs/gitoxide/commit/31e795abf27dea8fd7dd7f59996cdd5d70398601))
 * **Uncategorized**
    - Release gix-dir v0.7.0, gix-mailmap v0.23.6, gix-negotiate v0.14.0, gix-pack v0.52.0, gix-odb v0.62.0, gix-packetline v0.17.6, gix-transport v0.42.3, gix-protocol v0.45.3, gix-revision v0.28.0, gix-refspec v0.24.0, gix-status v0.12.0, gix-submodule v0.13.0, gix-worktree-state v0.12.0, gix v0.65.0, gix-fsck v0.5.0, gitoxide-core v0.40.0, gitoxide v0.38.0 ([`4fe330e`](https://github.com/GitoxideLabs/gitoxide/commit/4fe330e68d10d51b0a7116a7ef8b9ea3b48a224c))
    - Release gix-attributes v0.22.5, gix-filter v0.12.0, gix-fs v0.11.3, gix-revwalk v0.14.0, gix-traverse v0.40.0, gix-worktree-stream v0.14.0, gix-archive v0.14.0, gix-config-value v0.14.8, gix-tempfile v14.0.2, gix-ref v0.46.0, gix-sec v0.10.8, gix-config v0.39.0, gix-prompt v0.8.7, gix-url v0.27.5, gix-credentials v0.24.5, gix-ignore v0.11.4, gix-index v0.34.0, gix-worktree v0.35.0, gix-diff v0.45.0, gix-discover v0.34.0, gix-pathspec v0.7.7, gix-dir v0.7.0, gix-mailmap v0.23.6, gix-negotiate v0.14.0, gix-pack v0.52.0, gix-odb v0.62.0, gix-packetline v0.17.6, gix-transport v0.42.3, gix-protocol v0.45.3, gix-revision v0.28.0, gix-refspec v0.24.0, gix-status v0.12.0, gix-submodule v0.13.0, gix-worktree-state v0.12.0, gix v0.65.0, gix-fsck v0.5.0, gitoxide-core v0.40.0, gitoxide v0.38.0 ([`f2b522d`](https://github.com/GitoxideLabs/gitoxide/commit/f2b522df2ddad07f065f43c2dbad49aa726714dd))
    - Release gix-glob v0.16.5, gix-filter v0.12.0, gix-fs v0.11.3, gix-revwalk v0.14.0, gix-traverse v0.40.0, gix-worktree-stream v0.14.0, gix-archive v0.14.0, gix-config-value v0.14.8, gix-tempfile v14.0.2, gix-ref v0.46.0, gix-sec v0.10.8, gix-config v0.39.0, gix-prompt v0.8.7, gix-url v0.27.5, gix-credentials v0.24.5, gix-ignore v0.11.4, gix-index v0.34.0, gix-worktree v0.35.0, gix-diff v0.45.0, gix-discover v0.34.0, gix-pathspec v0.7.7, gix-dir v0.7.0, gix-mailmap v0.23.6, gix-negotiate v0.14.0, gix-pack v0.52.0, gix-odb v0.62.0, gix-packetline v0.17.6, gix-transport v0.42.3, gix-protocol v0.45.3, gix-revision v0.28.0, gix-refspec v0.24.0, gix-status v0.12.0, gix-submodule v0.13.0, gix-worktree-state v0.12.0, gix v0.65.0, gix-fsck v0.5.0, gitoxide-core v0.40.0, gitoxide v0.38.0 ([`a65a17f`](https://github.com/GitoxideLabs/gitoxide/commit/a65a17fc396ef49663b0a75cf7b5502d370db269))
    - Release gix-date v0.9.0, gix-actor v0.31.6, gix-validate v0.9.0, gix-object v0.43.0, gix-path v0.10.10, gix-attributes v0.22.4, gix-command v0.3.9, gix-packetline-blocking v0.17.5, gix-filter v0.12.0, gix-fs v0.11.3, gix-revwalk v0.14.0, gix-traverse v0.40.0, gix-worktree-stream v0.14.0, gix-archive v0.14.0, gix-ref v0.46.0, gix-config v0.39.0, gix-prompt v0.8.7, gix-url v0.27.5, gix-credentials v0.24.5, gix-ignore v0.11.4, gix-index v0.34.0, gix-worktree v0.35.0, gix-diff v0.45.0, gix-discover v0.34.0, gix-dir v0.7.0, gix-mailmap v0.23.6, gix-negotiate v0.14.0, gix-pack v0.52.0, gix-odb v0.62.0, gix-packetline v0.17.6, gix-transport v0.42.3, gix-protocol v0.45.3, gix-revision v0.28.0, gix-refspec v0.24.0, gix-status v0.12.0, gix-submodule v0.13.0, gix-worktree-state v0.12.0, gix v0.65.0, gix-fsck v0.5.0, gitoxide-core v0.40.0, gitoxide v0.38.0, safety bump 25 crates ([`d19af16`](https://github.com/GitoxideLabs/gitoxide/commit/d19af16e1d2031d4f0100e76b6cd410a5d252af1))
    - Prepare changelogs prior to release ([`0f25841`](https://github.com/GitoxideLabs/gitoxide/commit/0f2584178ae88e425f1c629eb85b69f3b4310d9f))
    - Merge branch 'fix-clean' ([`348b9bf`](https://github.com/GitoxideLabs/gitoxide/commit/348b9bf6cbf7a61b8094e5db8354376abb2e8c99))
    - Merge branch 'fix-clean' ([`33eacfb`](https://github.com/GitoxideLabs/gitoxide/commit/33eacfbaace2021043e2b5d72dcb9293af6c4020))
</details>

## 0.6.0 (2024-07-23)

### Bug Fixes

 - <csr-id-1e92d1ed3f23e9de84203252d64e701e2c7fb941/> assure that worktree-roots are never considered ignored.
   If they were, they would more easily be deleted by tooling like `gix clean`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 55 calendar days.
 - 62 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#1458](https://github.com/GitoxideLabs/gitoxide/issues/1458)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1458](https://github.com/GitoxideLabs/gitoxide/issues/1458)**
    - Assure that worktree-roots are never considered ignored. ([`1e92d1e`](https://github.com/GitoxideLabs/gitoxide/commit/1e92d1ed3f23e9de84203252d64e701e2c7fb941))
    - Add tests that show what happens with allow-lists and `/` in root ([`1ebd6c7`](https://github.com/GitoxideLabs/gitoxide/commit/1ebd6c71457b292cdef97c218fb3e4176d10d502))
 * **Uncategorized**
    - Release gix-actor v0.31.5, gix-filter v0.11.3, gix-fs v0.11.2, gix-commitgraph v0.24.3, gix-revwalk v0.13.2, gix-traverse v0.39.2, gix-worktree-stream v0.13.1, gix-archive v0.13.2, gix-config-value v0.14.7, gix-tempfile v14.0.1, gix-ref v0.45.0, gix-sec v0.10.7, gix-config v0.38.0, gix-prompt v0.8.6, gix-url v0.27.4, gix-credentials v0.24.3, gix-ignore v0.11.3, gix-index v0.33.1, gix-worktree v0.34.1, gix-diff v0.44.1, gix-discover v0.33.0, gix-pathspec v0.7.6, gix-dir v0.6.0, gix-mailmap v0.23.5, gix-negotiate v0.13.2, gix-pack v0.51.1, gix-odb v0.61.1, gix-transport v0.42.2, gix-protocol v0.45.2, gix-revision v0.27.2, gix-refspec v0.23.1, gix-status v0.11.0, gix-submodule v0.12.0, gix-worktree-state v0.11.1, gix v0.64.0, gix-fsck v0.4.1, gitoxide-core v0.39.0, gitoxide v0.37.0 ([`6232824`](https://github.com/GitoxideLabs/gitoxide/commit/6232824301847a9786dea0b926796a3187493587))
    - Release gix-glob v0.16.4, gix-attributes v0.22.3, gix-command v0.3.8, gix-filter v0.11.3, gix-fs v0.11.2, gix-commitgraph v0.24.3, gix-revwalk v0.13.2, gix-traverse v0.39.2, gix-worktree-stream v0.13.1, gix-archive v0.13.2, gix-config-value v0.14.7, gix-tempfile v14.0.1, gix-ref v0.45.0, gix-sec v0.10.7, gix-config v0.38.0, gix-prompt v0.8.6, gix-url v0.27.4, gix-credentials v0.24.3, gix-ignore v0.11.3, gix-index v0.33.1, gix-worktree v0.34.1, gix-diff v0.44.1, gix-discover v0.33.0, gix-pathspec v0.7.6, gix-dir v0.6.0, gix-mailmap v0.23.5, gix-negotiate v0.13.2, gix-pack v0.51.1, gix-odb v0.61.1, gix-transport v0.42.2, gix-protocol v0.45.2, gix-revision v0.27.2, gix-refspec v0.23.1, gix-status v0.11.0, gix-submodule v0.12.0, gix-worktree-state v0.11.1, gix v0.64.0, gix-fsck v0.4.1, gitoxide-core v0.39.0, gitoxide v0.37.0 ([`a1b73a6`](https://github.com/GitoxideLabs/gitoxide/commit/a1b73a67c19d9102a2c5a7f574a7a53a86d0094c))
    - Update manifests (by cargo-smart-release) ([`0470df3`](https://github.com/GitoxideLabs/gitoxide/commit/0470df3b8ebb136b219f0057f1e9a7031975cce5))
    - Prepare changelog prior to release ([`99c00cc`](https://github.com/GitoxideLabs/gitoxide/commit/99c00cc3ae9827555e2e1162328bc57038619d1f))
    - Merge branch 'fixes' ([`b4dba1c`](https://github.com/GitoxideLabs/gitoxide/commit/b4dba1c187baba44ee927daa538783f7f424b2f2))
    - Release gix-path v0.10.9 ([`15f1cf7`](https://github.com/GitoxideLabs/gitoxide/commit/15f1cf76764221d14afa66d03a6528b19b9c30c9))
    - Release gix-actor v0.31.4, gix-object v0.42.3 ([`bf3d82a`](https://github.com/GitoxideLabs/gitoxide/commit/bf3d82abc7c875109f9a5d6b6713ce68153b6456))
    - Merge branch 'heredocs' ([`7330844`](https://github.com/GitoxideLabs/gitoxide/commit/73308446e5ffee053af35b108e3d49c71db31e99))
    - Use `<<` rather than `<<-` heredoc operator ([`2641f8b`](https://github.com/GitoxideLabs/gitoxide/commit/2641f8b36008ade04d59d76bd6d546005ad76a21))
    - Release gix-path v0.10.8 ([`8d89b86`](https://github.com/GitoxideLabs/gitoxide/commit/8d89b865c84d1fb153d93343d1ce4e1d64e53541))
    - Merge branch 'tar-only' ([`1dfa90d`](https://github.com/GitoxideLabs/gitoxide/commit/1dfa90d641306b4099a6ecd52e2056b231467807))
    - Remove binary files in favor of `tar` files ([`dcab79a`](https://github.com/GitoxideLabs/gitoxide/commit/dcab79a6958cbf5cd69184c24497dc27c6f94961))
    - Merge branch 'main' into config-key-take-2 ([`9fa1054`](https://github.com/GitoxideLabs/gitoxide/commit/9fa1054a01071180d7b08c8c2b5bd61e9d0d32da))
    - Merge pull request #1361 from EliahKagan/freebsd ([`9c65d98`](https://github.com/GitoxideLabs/gitoxide/commit/9c65d9886328f53129b966aecdc91644297c54be))
    - Make bash script shebangs more portable ([`68cbea8`](https://github.com/GitoxideLabs/gitoxide/commit/68cbea815aa979acb0b86943db83ab77bbc728c4))
    - Release gix-fs v0.11.1, gix-glob v0.16.3 ([`2cefe77`](https://github.com/GitoxideLabs/gitoxide/commit/2cefe77203131878d0d8f5346f20f0e25b76cbea))
</details>

## 0.5.0 (2024-05-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 33 calendar days.
 - 33 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.38.2, gix-actor v0.31.2, gix-validate v0.8.5, gix-object v0.42.2, gix-command v0.3.7, gix-filter v0.11.2, gix-fs v0.11.0, gix-revwalk v0.13.1, gix-traverse v0.39.1, gix-worktree-stream v0.13.0, gix-archive v0.13.0, gix-tempfile v14.0.0, gix-lock v14.0.0, gix-ref v0.44.0, gix-config v0.37.0, gix-prompt v0.8.5, gix-index v0.33.0, gix-worktree v0.34.0, gix-diff v0.44.0, gix-discover v0.32.0, gix-pathspec v0.7.5, gix-dir v0.5.0, gix-macros v0.1.5, gix-mailmap v0.23.1, gix-negotiate v0.13.1, gix-pack v0.51.0, gix-odb v0.61.0, gix-transport v0.42.1, gix-protocol v0.45.1, gix-revision v0.27.1, gix-status v0.10.0, gix-submodule v0.11.0, gix-worktree-state v0.11.0, gix v0.63.0, gitoxide-core v0.38.0, gitoxide v0.36.0, safety bump 19 crates ([`4f98e94`](https://github.com/GitoxideLabs/gitoxide/commit/4f98e94e0e8b79ed2899b35bef40f3c30b3025b0))
    - Adjust changelogs prior to release ([`9511416`](https://github.com/GitoxideLabs/gitoxide/commit/9511416a6cd0c571233f958c165329c8705c2498))
    - Merge branch 'various-fixes' ([`d6cd449`](https://github.com/GitoxideLabs/gitoxide/commit/d6cd44930fb204b06e2b70fc6965e7705530c47a))
    - Merge pull request from GHSA-7w47-3wg8-547c ([`79dce79`](https://github.com/GitoxideLabs/gitoxide/commit/79dce79c62f6072aa2653780d590dc3993dfa401))
    - Address review comments ([`fcc3b69`](https://github.com/GitoxideLabs/gitoxide/commit/fcc3b69867db1628f6a44d0e0dad8f7417f566bc))
    - Adapt to changes in `gix-worktree` ([`1ca6a3c`](https://github.com/GitoxideLabs/gitoxide/commit/1ca6a3ce22887c7eb42ec3e0a19f6e1202715745))
    - Merge branch 'status' ([`68fd5b3`](https://github.com/GitoxideLabs/gitoxide/commit/68fd5b34e1214d5c2cc7d00dd06e19ee86c00c66))
    - Cleanup path classificaiton after fixes in `gix-pathspec` ([`44a2e00`](https://github.com/GitoxideLabs/gitoxide/commit/44a2e005ea8241d026ae542dd4a71cfb6cfd8308))
    - Merge branch 'cargo-fixes' ([`977346e`](https://github.com/GitoxideLabs/gitoxide/commit/977346ee61de6207c66f3de003db6e8c722fb81c))
</details>

## 0.4.1 (2024-04-18)

### Bug Fixes

 - <csr-id-7f2f3ff8adbecd631c2b4513995d6c94b21742eb/> assure worktree-roots aren't pruned with pathspecs that are never meant for them.
   Previously, when pathspecs were defined, the classification of the worktree-root
   would also be using them. This means that depending on the pathspec, worktree-roots would
   be pruned, which in turn makes it impossible to recurse into them.
   
   Now pathspecs are disabled when classifying the worktree-root directory.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 5 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-index v0.32.1, gix-pathspec v0.7.4, gix-worktree v0.33.1, gix-dir v0.4.1 ([`54ac559`](https://github.com/GitoxideLabs/gitoxide/commit/54ac55946bb04635cd74582a1ce2e4bee70f2e60))
    - Prepare changelog prior to `gix-dir` patch release ([`6ca6fa6`](https://github.com/GitoxideLabs/gitoxide/commit/6ca6fa69b5c21c8d8e9e07e21558e98201504cda))
    - Assure worktree-roots aren't pruned with pathspecs that are never meant for them. ([`7f2f3ff`](https://github.com/GitoxideLabs/gitoxide/commit/7f2f3ff8adbecd631c2b4513995d6c94b21742eb))
    - Merge pull request #1345 from EliahKagan/shell-scripts ([`fe24c89`](https://github.com/GitoxideLabs/gitoxide/commit/fe24c89e326670deaa3aaa643276d612d866072e))
    - Add missing +x bit on scripts that are run and not sourced ([`41bf65a`](https://github.com/GitoxideLabs/gitoxide/commit/41bf65adef6f7d2cdd28fede262173ec7ba10822))
</details>

## 0.4.0 (2024-04-13)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-trace v0.1.9, gix-utils v0.1.12, gix-packetline-blocking v0.17.4, gix-filter v0.11.1, gix-fs v0.10.2, gix-traverse v0.39.0, gix-worktree-stream v0.12.0, gix-archive v0.12.0, gix-config v0.36.1, gix-url v0.27.3, gix-index v0.32.0, gix-worktree v0.33.0, gix-diff v0.43.0, gix-pathspec v0.7.3, gix-dir v0.4.0, gix-pack v0.50.0, gix-odb v0.60.0, gix-transport v0.42.0, gix-protocol v0.45.0, gix-status v0.9.0, gix-worktree-state v0.10.0, gix v0.62.0, gix-fsck v0.4.0, gitoxide-core v0.37.0, gitoxide v0.35.0, safety bump 14 crates ([`095c673`](https://github.com/GitoxideLabs/gitoxide/commit/095c6739b2722a8b9af90776b435ef2da454c0e6))
    - Prepare changelogs prior to release ([`5755271`](https://github.com/GitoxideLabs/gitoxide/commit/57552717f46f96c35ba4ddc0a64434354ef845e9))
    - Merge pull request #1341 from szepeviktor/typos ([`55f379b`](https://github.com/GitoxideLabs/gitoxide/commit/55f379bc47065822d078393d83d30c0835a89782))
    - Fix typos ([`f72ecce`](https://github.com/GitoxideLabs/gitoxide/commit/f72ecce45babcad2a0c9b73c79d01ff502907a57))
    - Merge pull request #1334 from EliahKagan/nonstandard-worktree ([`37732fb`](https://github.com/GitoxideLabs/gitoxide/commit/37732fb13efdff5a1b8a836943e9e575196724b5))
    - Let nonstandard worktree fixtures work even if Git < 2.37.2 ([`01d6be9`](https://github.com/GitoxideLabs/gitoxide/commit/01d6be997fced8695b60d0dc206f87e542037a8d))
</details>

## 0.3.0 (2024-03-18)

### Bug Fixes

 - <csr-id-e7e91cfaed6d40a773a65fc077b99d2e26bb28f5/> allow traversals to start from a symlink that points to a directory
   Now symlinked repositories can be traversed as well.

### New Features (BREAKING)

 - <csr-id-35b74e7992a5a732b5ae8dbdc264479a91b1d60d/> allow directory walk to be interrupted with `should_interrupt` flag.
   That way, it can be much more responsive to interruption.
 - <csr-id-cd0c8af78fd7a4f06e33ec2ce06b094b5a490877/> assure symlinks to directories are ignored with `dir/` declarations in `.gitignore`.
   Initially, symlinks appear like symlinks thanks to `lstat`, but to do
   exclude handling correctly these need another `stat` call.
   
   However, this is also not done in Git, but done in `libgit2` only,
   so a toggle was added to act like Git by default, but allow obtaining
   the same behaviour as git2 for compatibility.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 3 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-actor v0.31.1, gix-object v0.42.1, gix-index v0.31.1, gix-pathspec v0.7.2, gix-dir v0.3.0, gix-status v0.8.0, gix v0.61.0, safety bump 2 crates ([`155cc45`](https://github.com/GitoxideLabs/gitoxide/commit/155cc45730b259e662d7c4be42a469a3af3750e1))
    - Prepare changelog prior to release ([`129ba3d`](https://github.com/GitoxideLabs/gitoxide/commit/129ba3deccc9ada0dc571466458845939502763d))
    - Merge branch 'improvements-for-cargo' ([`41cd53e`](https://github.com/GitoxideLabs/gitoxide/commit/41cd53e2af76e35e047aac4eca6324774df4cb50))
    - Allow directory walk to be interrupted with `should_interrupt` flag. ([`35b74e7`](https://github.com/GitoxideLabs/gitoxide/commit/35b74e7992a5a732b5ae8dbdc264479a91b1d60d))
    - Allow traversals to start from a symlink that points to a directory ([`e7e91cf`](https://github.com/GitoxideLabs/gitoxide/commit/e7e91cfaed6d40a773a65fc077b99d2e26bb28f5))
    - Assure symlinks to directories are ignored with `dir/` declarations in `.gitignore`. ([`cd0c8af`](https://github.com/GitoxideLabs/gitoxide/commit/cd0c8af78fd7a4f06e33ec2ce06b094b5a490877))
</details>

## 0.2.0 (2024-03-14)

### Bug Fixes

 - <csr-id-434f5434d7242f7f3d6b595f767195c51a3acd86/> make it possible to use a submodule root for a full walk.
   Previously, it would not allow to enter the repository, making
   a walk impossible.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 4 calendar days.
 - 18 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.8.5, gix-hash v0.14.2, gix-trace v0.1.8, gix-utils v0.1.11, gix-features v0.38.1, gix-actor v0.31.0, gix-validate v0.8.4, gix-object v0.42.0, gix-path v0.10.7, gix-glob v0.16.2, gix-quote v0.4.12, gix-attributes v0.22.2, gix-command v0.3.6, gix-filter v0.11.0, gix-fs v0.10.1, gix-chunk v0.4.8, gix-commitgraph v0.24.2, gix-hashtable v0.5.2, gix-revwalk v0.13.0, gix-traverse v0.38.0, gix-worktree-stream v0.11.0, gix-archive v0.11.0, gix-config-value v0.14.6, gix-tempfile v13.1.1, gix-lock v13.1.1, gix-ref v0.43.0, gix-sec v0.10.6, gix-config v0.36.0, gix-prompt v0.8.4, gix-url v0.27.2, gix-credentials v0.24.2, gix-ignore v0.11.2, gix-bitmap v0.2.11, gix-index v0.31.0, gix-worktree v0.32.0, gix-diff v0.42.0, gix-discover v0.31.0, gix-pathspec v0.7.1, gix-dir v0.2.0, gix-macros v0.1.4, gix-mailmap v0.23.0, gix-negotiate v0.13.0, gix-pack v0.49.0, gix-odb v0.59.0, gix-packetline v0.17.4, gix-transport v0.41.2, gix-protocol v0.44.2, gix-revision v0.27.0, gix-refspec v0.23.0, gix-status v0.7.0, gix-submodule v0.10.0, gix-worktree-state v0.9.0, gix v0.60.0, safety bump 26 crates ([`b050327`](https://github.com/GitoxideLabs/gitoxide/commit/b050327e76f234b19be921b78b7b28e034319fdb))
    - Prepare changelogs prior to release ([`52c3bbd`](https://github.com/GitoxideLabs/gitoxide/commit/52c3bbd36b9e94a0f3a78b4ada84d0c08eba27f6))
    - Merge branch 'status' ([`3e5c974`](https://github.com/GitoxideLabs/gitoxide/commit/3e5c974dd62ac134711c6c2f5a5490187a6ea55e))
    - Keep lower-bound of `thiserror` low in `gix-dir` ([`917634f`](https://github.com/GitoxideLabs/gitoxide/commit/917634fa694a1e91d37f6407e57ae96b3b0aec4b))
    - Fix lints for nightly, and clippy ([`f8ce3d0`](https://github.com/GitoxideLabs/gitoxide/commit/f8ce3d0721b6a53713a9392f2451874f520bc44c))
    - Make it possible to use a submodule root for a full walk. ([`434f543`](https://github.com/GitoxideLabs/gitoxide/commit/434f5434d7242f7f3d6b595f767195c51a3acd86))
</details>

## 0.1.0 (2024-02-25)

### Bug Fixes

 - <csr-id-95d10ee9371196cbcb8e599d28d9d05fa8b68221/> pathspec prefixes still allows directory collapsing.
 - <csr-id-dc200bf6f2cb10b6f0e45dd83bf9f82173cbb04f/> proper submodule handling
   Previously it was possible for `.git` files in directories to
   not trigger repository detection.
 - <csr-id-c04954a89dfdd8c230050b6175e2a132c73bdbfa/> assure `Action::Cancel` doesn't run into unreachable code.

### New Features (BREAKING)

 - <csr-id-bd5f44925306aa342b2b1c547779799b72372212/> Represent `DotGit` as `ExtendedKind`
   This cleans up the model despite also making it harder to detect
   whether something is a DotGit.
 - <csr-id-b6ea37a4d20e008c0b447090992c6aade0191265/> simplify `walk()` signature to compute `root` with pathspec directory.
   This makes the overall handling more unified, while assuring it's always
   in the worktree.
   
   And as a pathspec directory isn't exactly the same as a user-specified root,
   it's also possible to override this automation.
 - <csr-id-4567dbb2abf3d05bebe2206afafc40002a376d26/> allow to emit all collapsed entries.
   This is useful for rename tracking as it allows to see all files
   that may take part in a rename (i.e. when a directory is renamed).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 57 calendar days.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.8.4, gix-utils v0.1.10, gix-actor v0.30.1, gix-object v0.41.1, gix-path v0.10.6, gix-glob v0.16.1, gix-quote v0.4.11, gix-attributes v0.22.1, gix-command v0.3.5, gix-filter v0.10.0, gix-commitgraph v0.24.1, gix-worktree-stream v0.10.0, gix-archive v0.10.0, gix-config-value v0.14.5, gix-ref v0.42.0, gix-sec v0.10.5, gix-config v0.35.0, gix-prompt v0.8.3, gix-url v0.27.1, gix-credentials v0.24.1, gix-ignore v0.11.1, gix-index v0.30.0, gix-worktree v0.31.0, gix-diff v0.41.0, gix-discover v0.30.0, gix-pathspec v0.7.0, gix-dir v0.1.0, gix-pack v0.48.0, gix-odb v0.58.0, gix-transport v0.41.1, gix-protocol v0.44.1, gix-revision v0.26.1, gix-refspec v0.22.1, gix-status v0.6.0, gix-submodule v0.9.0, gix-worktree-state v0.8.0, gix v0.59.0, gix-fsck v0.3.0, gitoxide-core v0.36.0, gitoxide v0.34.0, safety bump 10 crates ([`45b4470`](https://github.com/GitoxideLabs/gitoxide/commit/45b447045bc826f252129c300c531acde2652c64))
    - Prepare changelogs prior to release ([`f2e111f`](https://github.com/GitoxideLabs/gitoxide/commit/f2e111f768fc1bc6182355261c20b63610cffec7))
    - Merge branch 'status' ([`d53504a`](https://github.com/GitoxideLabs/gitoxide/commit/d53504a1fad41cec7b6ca2a4abb7f185d8941e3f))
    - Make it even harder to remove your own CWD ([`4d5767c`](https://github.com/GitoxideLabs/gitoxide/commit/4d5767cd394d755104aa7f0c1ed5b8e01bf74b12))
    - Assure that we don't artificially make non-recursable directories visible ([`1a26732`](https://github.com/GitoxideLabs/gitoxide/commit/1a26732fe897161f9bfa397efdb07aa57f3c7341))
    - Represent `DotGit` as `ExtendedKind` ([`bd5f449`](https://github.com/GitoxideLabs/gitoxide/commit/bd5f44925306aa342b2b1c547779799b72372212))
    - Pathspec prefixes still allows directory collapsing. ([`95d10ee`](https://github.com/GitoxideLabs/gitoxide/commit/95d10ee9371196cbcb8e599d28d9d05fa8b68221))
    - Merge branch 'status' ([`bb48c4c`](https://github.com/GitoxideLabs/gitoxide/commit/bb48c4ce22650b8c76af3b147e252ebe7cedb205))
    - More natural top-level handling ([`44ccc67`](https://github.com/GitoxideLabs/gitoxide/commit/44ccc67a5b4a481f769399c41f0d3fc956fd8ec8))
    - Simplify `walk()` signature to compute `root` with pathspec directory. ([`b6ea37a`](https://github.com/GitoxideLabs/gitoxide/commit/b6ea37a4d20e008c0b447090992c6aade0191265))
    - Allow to emit all collapsed entries. ([`4567dbb`](https://github.com/GitoxideLabs/gitoxide/commit/4567dbb2abf3d05bebe2206afafc40002a376d26))
    - Proper submodule handling ([`dc200bf`](https://github.com/GitoxideLabs/gitoxide/commit/dc200bf6f2cb10b6f0e45dd83bf9f82173cbb04f))
    - Assure `Action::Cancel` doesn't run into unreachable code. ([`c04954a`](https://github.com/GitoxideLabs/gitoxide/commit/c04954a89dfdd8c230050b6175e2a132c73bdbfa))
    - Merge branch 'status' ([`b8def77`](https://github.com/GitoxideLabs/gitoxide/commit/b8def77e91ddc82a39ec342b89f558702a8f1d8c))
    - Make sure that `*foo*` prefixes don't end up matching any directory. ([`482d6f3`](https://github.com/GitoxideLabs/gitoxide/commit/482d6f3f773fd74ddcea4be0b36ebea89017397a))
    - Merge branch 'dirwalk' ([`face359`](https://github.com/GitoxideLabs/gitoxide/commit/face359443ba33e8985ec1525d5ec38b743ea7a9))
    - Implementation of the Git-style directory walk. ([`3252cfd`](https://github.com/GitoxideLabs/gitoxide/commit/3252cfd570b0c0897c51939e1a8c45b35c861c53))
    - Merge branch 'gix-status' ([`c3983c6`](https://github.com/GitoxideLabs/gitoxide/commit/c3983c6b8d63d85ec713ae8d661723f9cf0bd55b))
    - Initial version of the `gix-dir` crate ([`22acf0d`](https://github.com/GitoxideLabs/gitoxide/commit/22acf0def5c62563300aa8eaef01cb94bcd15645))
</details>

