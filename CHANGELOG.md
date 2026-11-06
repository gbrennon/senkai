# Changelog

All notable changes to this project are documented here.
## [0.1.0] - 2026-11-06


### Added
- Poll terminal input without consuming it ([e53f8d6](https://codeberg.org/gbrennon/senkai/commit/e53f8d68a57a5818f8e69c351632c873d70b29b2))
- Add opt-in multi-selection ([59d6358](https://codeberg.org/gbrennon/senkai/commit/59d6358fbe8b770a0f807bf853a74d3d29cee7f2))
- Confirm batch worktree removal ([e528805](https://codeberg.org/gbrennon/senkai/commit/e528805da9e6332209fea86b3e400008a54c9fab))
- Render multi-selection controls ([8c0369b](https://codeberg.org/gbrennon/senkai/commit/8c0369b52696e4350e3014776c6ec1e38fc3a7fe))
- Inherit terminal colors for loading widget ([c1f453a](https://codeberg.org/gbrennon/senkai/commit/c1f453aa1925e261a7e98d92862cb3962f8cbfa2))
- Add core ShellQuoter module ([1dd2298](https://codeberg.org/gbrennon/senkai/commit/1dd2298ade84fef851126404b9c32db5ee865e68))
- Migrate to workspace crates and implement clap cli ([3d13a60](https://codeberg.org/gbrennon/senkai/commit/3d13a606bfa4bdba680204eb2656c1e52c5b905a))
- Introduce DI Container in infrastructure crate ([ad22da7](https://codeberg.org/gbrennon/senkai/commit/ad22da72d956d0ebcf61ab8cfb743c5aea4487fb))
- Introduce CompositionRoot and Application ([9435d43](https://codeberg.org/gbrennon/senkai/commit/9435d4350eba541b25b70ac9516d29dfd8a10421))
- Add shared color palette module ([2e61192](https://codeberg.org/gbrennon/senkai/commit/2e61192bb7cd2113f0e30584c4ed221213aa368f))
- Export theme module ([1ad9bf1](https://codeberg.org/gbrennon/senkai/commit/1ad9bf19179af90821ca9d9f258c3b5627618fcd))
- Apply shared palette to selector styles ([557ead1](https://codeberg.org/gbrennon/senkai/commit/557ead171f098d85169126ec2d6b9e5c22e22232))
- Apply shared palette to loading widget ([452692f](https://codeberg.org/gbrennon/senkai/commit/452692f9f81aa60a82632656b5055547a777ccfe))


### Changed
- Split struct modules ([a4f65a3](https://codeberg.org/gbrennon/senkai/commit/a4f65a33521698987504f5dd46e7872adf870027))
- Add repository port module ([151ba44](https://codeberg.org/gbrennon/senkai/commit/151ba44c9a5ead19f1ebde859364f25f99dfa0b8))
- Add tmux port module ([3fe868a](https://codeberg.org/gbrennon/senkai/commit/3fe868a7a4bfb24323323b25fcf61f2180574cea))
- Remove legacy ports file ([c525909](https://codeberg.org/gbrennon/senkai/commit/c52590993f459eeac58e67ccf036b1579f616029))
- Add ports module ([fe248c2](https://codeberg.org/gbrennon/senkai/commit/fe248c26c639f71fa443449872ce1ff0f0682df6))
- Migrate error.rs ([7d0c369](https://codeberg.org/gbrennon/senkai/commit/7d0c369809f96f99b5701cd1a638df8de66c23bd))
- Migrate project_locator.rs ([784e066](https://codeberg.org/gbrennon/senkai/commit/784e06624453d52baf32a82e124ad7f1171b804c))
- Migrate git_executor.rs ([42ab019](https://codeberg.org/gbrennon/senkai/commit/42ab0191a5ef04a2ea17197b42bea428ad793489))
- Migrate tmux_executor.rs ([74d9849](https://codeberg.org/gbrennon/senkai/commit/74d9849f5a34c75f6b19c7b7a3adac29f6becb95))
- Migrate cli.rs ([06dc879](https://codeberg.org/gbrennon/senkai/commit/06dc879e5e244d32f4f8cc1e084c7e39ec3509cb))
- Migrate git fake ([5c40d0f](https://codeberg.org/gbrennon/senkai/commit/5c40d0fc267e0be8ca122ac4cf8a91f518796c8f))
- Migrate tmux fake ([7d2bcf1](https://codeberg.org/gbrennon/senkai/commit/7d2bcf18ea3dfba0aecbd02782dbf55274b3b519))
- Migrate external worktree test ([c43c1d7](https://codeberg.org/gbrennon/senkai/commit/c43c1d7f612a7b7e013d707b701a7a6a24d8e83f))
- Migrate lifecycle test ([6a69c64](https://codeberg.org/gbrennon/senkai/commit/6a69c6481ea5eceb29fdbb9d3e196eea9882a00b))
- Migrate cli test ([1503c7d](https://codeberg.org/gbrennon/senkai/commit/1503c7d036fd4241420337d0c9bd1454e8d0ad0f))
- Expose shell_quoter module in core ([1e614b5](https://codeberg.org/gbrennon/senkai/commit/1e614b5e941db14ea82ffbb914a3b00f3e0ee04f))
- Remove ShellQuoter port trait ([fbf5e1f](https://codeberg.org/gbrennon/senkai/commit/fbf5e1fd86bcab85cc3a50dc04381afe305d1fca))
- Inline ShellQuoter directly in git executor ([f83e87f](https://codeberg.org/gbrennon/senkai/commit/f83e87f7e184916e30d8bcc62a8f3ce07ce32bb1))
- Inline ShellQuoter directly in tmux executor ([3d8830b](https://codeberg.org/gbrennon/senkai/commit/3d8830b1d447e70da7b26d98e176a43cdaa44323))
- Remove ShellQuoter utility module ([23a3abb](https://codeberg.org/gbrennon/senkai/commit/23a3abb3380077d6dd411c5ba7915da04e92abde))
- Migrate cli ShellQuoter to core shell_quoter module ([83e9885](https://codeberg.org/gbrennon/senkai/commit/83e98852c530aa59116e65581d7637a7deb463a6))
- Relocate main crate to root of src dir ([c32ee28](https://codeberg.org/gbrennon/senkai/commit/c32ee2853606c651e640e7f75719d6c7fb11023e))
- Assemble Application via CompositionRoot and Container ([92c739c](https://codeberg.org/gbrennon/senkai/commit/92c739c3e00333d4130f603e0394d42d9517d869))
- Rename workspace crates to senkai and reset version to 0.1.0 ([3505ddd](https://codeberg.org/gbrennon/senkai/commit/3505dddd2fc35e05672f906ebb76dd561e24e25a))
- Rename tmux_port to multiplexer_port ([76f5d92](https://codeberg.org/gbrennon/senkai/commit/76f5d920ced80dde5622f58cc4f1fe4b08cb4f37))
- Update runtime errors, project root env, and cli tests to senkai ([96d1767](https://codeberg.org/gbrennon/senkai/commit/96d176767fded796fd6ae166e3df831975c980c3))
- Package and install the sk binary ([bf547db](https://codeberg.org/gbrennon/senkai/commit/bf547db615b3e38989cdadcab7ad01dff044f87a))


### Documentation
- Design input-aware loading animation ([eeab8dc](https://codeberg.org/gbrennon/senkai/commit/eeab8dc2149485be6aadad8d93bafe71b0d5088f))
- Plan input-aware loading animation ([8699d64](https://codeberg.org/gbrennon/senkai/commit/8699d646e01eb5f182658af0c098590733d0de8e))
- Clarify loading prompt focus ([c04b0b0](https://codeberg.org/gbrennon/senkai/commit/c04b0b076c666d2067ed7af29f713cc40a704f56))
- Document multi-select removal ([8e1a0ee](https://codeberg.org/gbrennon/senkai/commit/8e1a0eefd449fb5c27d6d4df33db2de1bcb56c62))
- Describe cleanup picker ([d908339](https://codeberg.org/gbrennon/senkai/commit/d9083396bc0ed4e7afa594b87a522cbd50e1a735))
- Document installed CLI resolution ([b73a959](https://codeberg.org/gbrennon/senkai/commit/b73a95949eab83362cd4581dbc6cf192620020e3))
- List default CLI paths ([43e2187](https://codeberg.org/gbrennon/senkai/commit/43e2187b82111c1a9c0db49960f45178aeb2194b))
- Document the sk binary and asset name ([f73e8f7](https://codeberg.org/gbrennon/senkai/commit/f73e8f774b68e50914e43420c7ba14d1de5a85f0))
- Clarify cli and multiplexer roles ([571a457](https://codeberg.org/gbrennon/senkai/commit/571a457d3654213942fa40ceb1850f6afb4b2133))
- Explain Bleach naming origin ([b952338](https://codeberg.org/gbrennon/senkai/commit/b952338267f42647b73a237ec3fe7cd61f850566))


### Fixed
- Hide animation during terminal input ([7c355ff](https://codeberg.org/gbrennon/senkai/commit/7c355ff0221237b48a944e14281dcfc8ae6627e5))
- Inherit terminal theme colors ([3630c7e](https://codeberg.org/gbrennon/senkai/commit/3630c7eeca26672eeae55cfb3420ec76e1e859e1))
- Prefer installed CLI binary ([d90c14c](https://codeberg.org/gbrennon/senkai/commit/d90c14c05e437d6638c2b2c36d8984f61b6dfba1))
- Prefer default CLI paths ([1643e4f](https://codeberg.org/gbrennon/senkai/commit/1643e4f997803007b2c92ae54c2eeeded95d0b23))
- Restore selector colors ([b5a444a](https://codeberg.org/gbrennon/senkai/commit/b5a444a78004a6861df0bfa15a6e257b8d979546))
- Restore loading colors ([ad9b652](https://codeberg.org/gbrennon/senkai/commit/ad9b652e24c4bba8cbb865c2a2fb037eacb29536))
- Prefer PATH CLI before fallback paths ([30943f9](https://codeberg.org/gbrennon/senkai/commit/30943f96430f3fb18a074cb3b8d929a2eb5bc7c7))
- Allow lowercase merge in commit message check ([c34a80a](https://codeberg.org/gbrennon/senkai/commit/c34a80ad6a39a8c55e61fb29ae81fec1670fef03))


### Maintenance
- Remove unused selector test import ([b5286f2](https://codeberg.org/gbrennon/senkai/commit/b5286f21e1696605e2aba98bd9133a79569698e7))
- Add semgrep architecture rules ([22fc2aa](https://codeberg.org/gbrennon/senkai/commit/22fc2aaf19150ec93ee94ca6b66ebaa99075cf4b))
- Remove dead e2e test scaffolding consumed by nothing ([ad751a0](https://codeberg.org/gbrennon/senkai/commit/ad751a095296ed25e107bc784920408bcb084e6c))
- Run commit check test in test-release-scripts ([166197b](https://codeberg.org/gbrennon/senkai/commit/166197bc072da3b82bae4e8c31d1ac459278bbff))
- Consolidate pending worktree changes ([0dc4af9](https://codeberg.org/gbrennon/senkai/commit/0dc4af9b48295b05bfac182e24c38ebb14fec8f3))
- Finalize worktree consolidation ([5af0fdc](https://codeberg.org/gbrennon/senkai/commit/5af0fdcf92900930280bd8fc7cc315ebdaf7465c))
- Add version constraints to root path dependencies ([34e6d2b](https://codeberg.org/gbrennon/senkai/commit/34e6d2b28dc33196000dcd85805af0beb230a715))
- Add version constraint to infra path dependency ([4cf50d6](https://codeberg.org/gbrennon/senkai/commit/4cf50d6ac72e578ed9f4235bbf1395494bb973b0))
- Add version constraint to presentation path dependency ([0c73bd1](https://codeberg.org/gbrennon/senkai/commit/0c73bd18dd97994c398e604a5da32c3142255abc))
- Publish all workspace members in dependency order ([cc9531d](https://codeberg.org/gbrennon/senkai/commit/cc9531da581f6bf2a8bafc69a7a49ff96cad91c9))
- Dry-run publish all workspace members ([217dca8](https://codeberg.org/gbrennon/senkai/commit/217dca8037dc705889b6ad4125076afd69c04936))
- Assert all four workspace members publish in order ([eba5134](https://codeberg.org/gbrennon/senkai/commit/eba5134ae4b14e3144a844ac573d56c99374b3ca))
- Add package metadata to senkai-core ([f548b23](https://codeberg.org/gbrennon/senkai/commit/f548b23c160f7b9b096e39cd2aa56f570f80165f))
- Add package metadata to senkai-infrastructure ([c282585](https://codeberg.org/gbrennon/senkai/commit/c28258528ce10372b46b05dab5222dc5561f9b65))
- Add package metadata to senkai-presentation ([95e3794](https://codeberg.org/gbrennon/senkai/commit/95e379483d1cf0c29b5d67da0e01a69bae25d61f))
- Validate core publish plus dependent manifests preflight ([ba0e766](https://codeberg.org/gbrennon/senkai/commit/ba0e766c602354c5d4eb2de488c12db0c45e718e))
- Validate dependent manifests carry publish metadata ([1919a65](https://codeberg.org/gbrennon/senkai/commit/1919a65083d3b27c5c0bdc573694ed8c5c1e2e1e))
- Collapse changelog to unreleased ([2b88267](https://codeberg.org/gbrennon/senkai/commit/2b88267d1dc6abb2e4d97e84df7b5c96f5ae9642))


### Other
- Merge pull request 'fix(loading): hide animation during terminal input' (#47) from feat/pause-loading-on-input into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/47 ([25fd6f9](https://codeberg.org/gbrennon/senkai/commit/25fd6f9fe317767f3e4435d3eee497c4fbfdfc6e))
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([f8bd908](https://codeberg.org/gbrennon/senkai/commit/f8bd908615df2c1d4e8cc735d4a2d6b014839b3f))
- Merge pull request 'feat(cleanup): support confirmed multi-worktree removal' (#48) from feat/remove-worktree-multiselect into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/48 ([c84b010](https://codeberg.org/gbrennon/senkai/commit/c84b0109254234c74e9599c187a2ca5606a40928))
- Merge pull request 'fix(plugin): use installed CLI binary' (#49) from fix/use-installed-cli into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/49 ([3044e5c](https://codeberg.org/gbrennon/senkai/commit/3044e5c0433ec206e672217265c56397f02d9781))
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([97fd8d0](https://codeberg.org/gbrennon/senkai/commit/97fd8d0212076573ba242d633d6aead325b2ace8))
- Merge pull request 'fix(plugin): prefer Cargo default binary' (#50) from fix/prefer-default-cli into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/50 ([3e67748](https://codeberg.org/gbrennon/senkai/commit/3e677483fcfb6487a8ad9b3ee77764a8f846e0f9))
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([6b663cf](https://codeberg.org/gbrennon/senkai/commit/6b663cf8a85dc7a56e99a3137b73102ca69fb303))
- Merge pull request 'fix(tui): restore selector and loading colors' (#51) from fix/inherit-terminal-colors into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/51
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([dd66d55](https://codeberg.org/gbrennon/senkai/commit/dd66d55fe4c55c5bae46899ae57409c38c16a052))
- Merge pull request 'fix(installer): prefer PATH CLI before fallback paths' (#52) from fix/installer-path-resolution into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/52 ([58c7e05](https://codeberg.org/gbrennon/senkai/commit/58c7e05b08ed4a5bf512b52738baab27949cc473))
- Merge pull request 'chore: add Semgrep architecture rules' (#53) from chore/add-semgrep-config into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/53 ([18f5323](https://codeberg.org/gbrennon/senkai/commit/18f53231b9bc3cf2e7fdc9c31a0ba4ec51692e1d))
- Isolate semgrep installation ([84f0f85](https://codeberg.org/gbrennon/senkai/commit/84f0f854d53c03a91bac1f05b6f00dfb465e547d))
- Merge pull request 'ci: add Semgrep architecture checks' (#55) from chore/add-semgrep-ci into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/55 ([6033dc7](https://codeberg.org/gbrennon/senkai/commit/6033dc726f78af45fa0352f4be7881670862530e))
- Merge pull request 'refactor: split core ports into focused modules' (#56) from chore/split-command-types into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/56 ([f1fbae9](https://codeberg.org/gbrennon/senkai/commit/f1fbae9fef02d712a713d56c71c4ba629898472e))
- Merge PR #57: shell_quoter refactor and port/cli cleanup

- rename src/utils.rs -> src/core/shell_quoter.rs
- remove unused ShellQuoter import from cli.rs
- delete tests/e2e/worktree_lifecycle_test.rs list_worktree_names_empty_initially()
- tighten git/tmux executor tests
- remove dead cli/command test lines

230 tests pass. ([c8c5037](https://codeberg.org/gbrennon/senkai/commit/c8c5037392ccd6a89abbe90eea0c9095f75fc3e3))
- Merge pull request 'fix(ci): allow lowercase merge in commit message check' (#58) from fix/merge-pipeline-commit-check into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/58 ([2831613](https://codeberg.org/gbrennon/senkai/commit/28316137537d4e152d266368b1ea6a3b51a3f247))
- Merge branch 'main' of ssh://codeberg.org/gbrennon/tmux-worktrees ([d138236](https://codeberg.org/gbrennon/senkai/commit/d13823652e4e893e36062b16f38ab1c596c912ff))
- Merge pull request 'feat(workspace): migrate to workspace crates and implement clap cli' (#59) from feat/workspace-crates-clap-cli into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/59 ([044af8d](https://codeberg.org/gbrennon/senkai/commit/044af8ddf17e59d3c3cf1428f4e99ebc89c8f0b1))
- Merge branch 'main' of ssh://codeberg.org/gbrennon/tmux-worktrees ([50b1aa2](https://codeberg.org/gbrennon/senkai/commit/50b1aa2bb09ddff4c1a870f7fbdc265e28db1586))
- Merge pull request 'refactor(crate): relocate main crate to root of src dir' (#60) from chore/root-main-crate into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/60 ([d0275b6](https://codeberg.org/gbrennon/senkai/commit/d0275b6a5d09e1d58070a0023c9a2f441559d282))
- Merge branch 'main' of ssh://codeberg.org/gbrennon/tmux-worktrees ([28916a3](https://codeberg.org/gbrennon/senkai/commit/28916a34be9bdfcf93badace743a2ac10868a93c))
- Merge pull request 'refactor: relocate DI container to infrastructure and wire via presentation CompositionRoot' (#61) from chore/infra-container-presentation-composition-root into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/61
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([bca5eb4](https://codeberg.org/gbrennon/senkai/commit/bca5eb418ae5fe31b6402f498eaed22672a3cc90))
- Merge pull request 'refactor(crate): rename workspace crates to senkai and reset version to 0.1.0' (#62) from chore/rebrand-rename-crates into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/62 ([a923855](https://codeberg.org/gbrennon/senkai/commit/a923855e9ba7d779e1205fadbaac00be81bac290))
- Merge pull request 'refactor: complete senkai rebrand' (#68) from chore/adjust-old-names-docs into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/68 ([5416f63](https://codeberg.org/gbrennon/senkai/commit/5416f63825a432177d2590a81ea430986a730434))
- Merge pull request 'chore: consolidate pending worktree changes' (#69) from feat/consolidate-worktree-changes into main

Reviewed-on: https://codeberg.org/gbrennon/senkai/pulls/69 ([81b49a5](https://codeberg.org/gbrennon/senkai/commit/81b49a52a8780a485f6f6fe315871b59c66a57ff))
- Merge pull request 'docs(readme): clarify cli and multiplexer roles' (#71) from chore/clarify-cli-and-multiplexer into main

Reviewed-on: https://codeberg.org/gbrennon/senkai/pulls/71 ([0cea78a](https://codeberg.org/gbrennon/senkai/commit/0cea78a59a27a623801350f5a47f976828e19441))
- Merge pull request 'docs(readme): explain Bleach naming origin' (#72) from chore/explain-senkai-bleach-origin into main

Reviewed-on: https://codeberg.org/gbrennon/senkai/pulls/72 ([97b4aab](https://codeberg.org/gbrennon/senkai/commit/97b4aab08aa11aad69f7c2efe2d40861c6d213e7))
- Merge pull request 'feat/shared-tui-palette' (#73) from feat/shared-tui-palette into main

Reviewed-on: https://codeberg.org/gbrennon/senkai/pulls/73 ([4b32459](https://codeberg.org/gbrennon/senkai/commit/4b32459880f69bf46045cdbb42cd48e204d63ea3))
- Merge pull request 'chore/fix-workspace-publish' (#74) from chore/fix-workspace-publish into main

Reviewed-on: https://codeberg.org/gbrennon/senkai/pulls/74 ([4558dbe](https://codeberg.org/gbrennon/senkai/commit/4558dbe9ddb6e3dceac7965f9a9bd419ab8a1399))
- Merge pull request 'chore: collapse changelog to unreleased' (#77) from chore/changelog-unreleased into main

Reviewed-on: https://codeberg.org/gbrennon/senkai/pulls/77 ([19f9a02](https://codeberg.org/gbrennon/senkai/commit/19f9a02fc4ace46326fb03541ea77c5d9e0aedfb))


### Tests
- Cover input-paused animation states ([b483fe8](https://codeberg.org/gbrennon/senkai/commit/b483fe8a0e93a2abbc073972f585aede893a1242))
- Model ordered Git outcomes ([703aca5](https://codeberg.org/gbrennon/senkai/commit/703aca5631010a929ed3443af37ffbbfb8816974))
- Record fake selector calls ([8b95c2c](https://codeberg.org/gbrennon/senkai/commit/8b95c2c0879fab60d7a88945f154b4466b618912))
- Record killed windows ([bcb9ad1](https://codeberg.org/gbrennon/senkai/commit/bcb9ad149c91d1eae3cc4339169bc71f8a9b866e))
- Register cleanup scenario ([c85d4f4](https://codeberg.org/gbrennon/senkai/commit/c85d4f4ef26cf317d7bb17d0ca5498e5227f48b8))
- Adapt branch deletion fake ([6a939cb](https://codeberg.org/gbrennon/senkai/commit/6a939cb2577493373f876228d3cc107e20ebe2c9))
- Exercise cleanup through real tmux ([252b6ab](https://codeberg.org/gbrennon/senkai/commit/252b6abd5f209d749e2258d50c1bb9bea5de3f8f))
- Cover batch failure and confirmation ([5cfa667](https://codeberg.org/gbrennon/senkai/commit/5cfa667d84d05551c93fafdbca9b8f3545e9cd0c))
- Cover installed CLI resolution ([ed26863](https://codeberg.org/gbrennon/senkai/commit/ed2686350c4b50a5b94ed9fae5c71a47cc3006f3))
- Cover plugin path precedence ([a9e4e5d](https://codeberg.org/gbrennon/senkai/commit/a9e4e5dbf27b6a65404ed8bd95a54d1506172a5d))
- Remove list_worktree_names test from lifecycle test ([338723b](https://codeberg.org/gbrennon/senkai/commit/338723bc1b5b85f1edc658e8caf1cbc672997a18))
- Use CommandRunner trait bound in git executor test helper ([67c48a6](https://codeberg.org/gbrennon/senkai/commit/67c48a622038956bc7d30756b201456a59e57c9e))
- Migrate tmux executor test ShellQuoter to core module ([8dd91d0](https://codeberg.org/gbrennon/senkai/commit/8dd91d0fe2dce13fb3a49bf488c6d139b51fd6e7))
- Remove list_worktree_names tests from cli test ([3e180f2](https://codeberg.org/gbrennon/senkai/commit/3e180f2d08cd2151712a07dc2ed49a7001664f09))
- Remove as_str_returns_correct_variants test from command test ([4b14607](https://codeberg.org/gbrennon/senkai/commit/4b1460759c9cb276a209821e4d227731e013b1ee))
- Add commit message check test suite ([7dc69d8](https://codeberg.org/gbrennon/senkai/commit/7dc69d8c1fca0145ec6f0774a002c5823a866d88))

## [0.5.0] - 2026-10-24


### Added
- Add selector popup colors ([29b50df](https://codeberg.org/gbrennon/senkai/commit/29b50dfb43cc2f7e314c0f97b017626aae812556))
- Support vim selector navigation ([8146002](https://codeberg.org/gbrennon/senkai/commit/8146002576a558612e09818c66cc074bc07587bc))


### Changed
- Use worktree terminology ([e7af169](https://codeberg.org/gbrennon/senkai/commit/e7af1695af8693854081f10bb89412020cbedba6))


### Fixed
- Position selector input cursor ([da42d0e](https://codeberg.org/gbrennon/senkai/commit/da42d0ed47e6e2403f6e041024cf0523f477ca6f))
- Require worktree context for popup ([17b42a9](https://codeberg.org/gbrennon/senkai/commit/17b42a9768594dc1ccfd9e210a5ec08db39f1423))
- Persist SSH agent for tmux ([68e68df](https://codeberg.org/gbrennon/senkai/commit/68e68df74a3cf9749ff778f9d94b118d398c27a5))
- Allow main repository popup ([b42b10a](https://codeberg.org/gbrennon/senkai/commit/b42b10af55c557eb5f81a49f9724a69d27d3f590))


### Maintenance
- Prepare v0.5.0 ([28acda8](https://codeberg.org/gbrennon/senkai/commit/28acda86cbccd5d4f5e330b17c1a04b977b70508))
- Update changelog ([d9be00a](https://codeberg.org/gbrennon/senkai/commit/d9be00a793332f3a52ffb85ed79406ecad0340ee))
- Rerun release checks ([5112d1a](https://codeberg.org/gbrennon/senkai/commit/5112d1ad9d955dc1f16d314822d6dedcae42c4b3))


### Other
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([a537e6a](https://codeberg.org/gbrennon/senkai/commit/a537e6ac1180ac44ca83d81ca82d24c4de3d883c))
- Merge pull request 'fix(ui): position selector input cursor' (#39) from fix/selector-cursor-marker into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/39⁩ ([547b621](https://codeberg.org/gbrennon/senkai/commit/547b621125d90cddbfd58f5afd356bb5b308e238))
- Merge pull request 'refactor(ui): use worktree terminology' (#40) from fix/worktree-naming into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/40⁩ ([8909b31](https://codeberg.org/gbrennon/senkai/commit/8909b316d13e95098eaac40184f89891a39e8459))
- Merge pull request 'feat(ui): add selector popup colors' (#41) from feat/ratatui-colors into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/41⁩ ([d7ab784](https://codeberg.org/gbrennon/senkai/commit/d7ab784154a453aae635070798ee33ff4836d387))
- Merge pull request 'fix(ui): require worktree context for popup' (#42) from fix/popup-close-error into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/42⁩ ([67e0cb0](https://codeberg.org/gbrennon/senkai/commit/67e0cb0e3655bc0e3c1fd76023836d9ebded91f0))
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([c6af3f7](https://codeberg.org/gbrennon/senkai/commit/c6af3f7d5727fbd47b259ab6fa23a45db10263d0))
- Merge pull request 'fix(auth): persist SSH agent for tmux' (#43) from fix/persist-ssh-agent into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/43 ([5858ee3](https://codeberg.org/gbrennon/senkai/commit/5858ee33f364117960bf9fa053bca2d5c38c6df3))
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([f300be5](https://codeberg.org/gbrennon/senkai/commit/f300be5eb3485e0df3ee7d974485678a1465116b))
- Merge pull request 'fix(ui): allow main repository popup' (#44) from fix/allow-main-repository into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/44 ([33e15af](https://codeberg.org/gbrennon/senkai/commit/33e15afe2a33ed8b7fbe1a81a0e028ef3b3b3c26))
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([3939d05](https://codeberg.org/gbrennon/senkai/commit/3939d058aea31927526717ae204d03ffccbb7408))
- Merge pull request 'feat(ui): support ctrl-j and ctrl-k selector navigation' (#45) from feat/vim-selector-navigation into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/45 ([e122845](https://codeberg.org/gbrennon/senkai/commit/e122845fffb81886dcce70d94a820a584f5ab71f))
- Merge pull request 'release: v0.5.0' (#46) from release/v0.5.0 into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/46 ([bac6e64](https://codeberg.org/gbrennon/senkai/commit/bac6e64b07033ca3349d78ca54d213e0afbd0f3d))


### Tests
- Fix selector key assertions ([4fd05fa](https://codeberg.org/gbrennon/senkai/commit/4fd05faf44888376760999ffb15a024abbd1ce58))

## [0.4.0] - 2026-10-22


### Added
- Download binaries from GitHub releases ([d6973bb](https://codeberg.org/gbrennon/senkai/commit/d6973bb8f9563f2968d7447a8edbdb91509e2ad9))


### Fixed
- Parse GitHub release metadata ([7413fbb](https://codeberg.org/gbrennon/senkai/commit/7413fbbd024c435d5ad0e098a2f6756d97967f75))
- Ignore generated release assets ([a570866](https://codeberg.org/gbrennon/senkai/commit/a57086685295fa582ad7d63b55f3a46381da4f10))


### Maintenance
- Prepare v0.4.0 ([4af4aca](https://codeberg.org/gbrennon/senkai/commit/4af4aca44d2631ed13adf5f26954bb204841ddf1))
- Update changelog ([7131fbe](https://codeberg.org/gbrennon/senkai/commit/7131fbe2b940815fb069bdccf223e89dd8e6570b))
- Update changelog ([3d5fcca](https://codeberg.org/gbrennon/senkai/commit/3d5fcca87da2974251b519ad91d521fa102b5ca6))


### Other
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([0ca60cb](https://codeberg.org/gbrennon/senkai/commit/0ca60cbc2ff87dec7c597028baae0a4b269a7b39))
- Merge pull request 'feat: download binaries from GitHub releases' (#36) from feat/github-release-assets into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/36⁩ ([66cbca8](https://codeberg.org/gbrennon/senkai/commit/66cbca87463123a613c21d03865eb8a17d9a6e72))
- Verify release pull request checks ([94ae0c7](https://codeberg.org/gbrennon/senkai/commit/94ae0c79c6db6c2f860559bed719c99b9d85f77b))
- Merge pull request 'release: v0.4.0' (#37) from release/v0.4.0 into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/37⁩ ([194a66a](https://codeberg.org/gbrennon/senkai/commit/194a66afd5ceb6df0145a35de321ec2abae0fd13))
- Merge pull request 'fix(release): ignore generated assets' (#38) from release/v0.4.0 into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/38⁩ ([533e4a4](https://codeberg.org/gbrennon/senkai/commit/533e4a4b5e31ab3e2d447489253117f044f50f0b))

## [0.3.0] - 2026-10-21


### Added
- Integrate tmux window kill into remove_worktree ([f8e094c](https://codeberg.org/gbrennon/senkai/commit/f8e094cd79cbfe09bcbf60ddcee45dc81a9d2d28))
- Wrap git subprocess calls ([93446db](https://codeberg.org/gbrennon/senkai/commit/93446dba37ebbd187fce715829cdec2f2f61cb71))
- Resolve repo root and ensure worktrees dir ([edee67c](https://codeberg.org/gbrennon/senkai/commit/edee67c26d095901271824e57d868aa928f991f7))
- List, remove, and delete worktree branches ([e6c1449](https://codeberg.org/gbrennon/senkai/commit/e6c14498e2b3ed1abd7305b2ba4f51769d19e2ce))
- Detect merged branches ([92609c6](https://codeberg.org/gbrennon/senkai/commit/92609c659a3234121d8be5af3e9ae1e82c0fef1f))
- Add tmux option and window integration ([816b7f9](https://codeberg.org/gbrennon/senkai/commit/816b7f9bfad2ecce71e39688d8575a3562c65c47))
- Add ratatui worktree picker ([46424a5](https://codeberg.org/gbrennon/senkai/commit/46424a558219f5866f4a61853d1603142fdc3987))
- Dispatch subcommands and spawn popup ([39fa5e8](https://codeberg.org/gbrennon/senkai/commit/39fa5e849c38feda9b82022d0ce45ebfd8a45c08))
- Add core module with pure logic ([3758ccc](https://codeberg.org/gbrennon/senkai/commit/3758ccc3739bbb17c850af0b916ee5784db0856f))
- Add infrastructure module with git/tmux executors ([ac6079b](https://codeberg.org/gbrennon/senkai/commit/ac6079bf41c5dca78bf3fd04d91ae9bbc0e02fd4))
- Add justfile with build, test, install, and coverage recipes ([257dcdb](https://codeberg.org/gbrennon/senkai/commit/257dcdb98e0e70df6b06bbfb52e15dee6a4c5550))
- Add justfile with build, test, install, and coverage recipes ([4c4c22a](https://codeberg.org/gbrennon/senkai/commit/4c4c22a0d1861511fd95c46299029009f41810fc))
- Add MergeChecker for remote merge verification ([d5a860a](https://codeberg.org/gbrennon/senkai/commit/d5a860a0094e8da99d611552865abf1d3d8828ed))
- Add ProjectLocator with env, walk, and tmux strategies ([9349207](https://codeberg.org/gbrennon/senkai/commit/934920775b10e1afde3b8776b97a13985a806905))
- Add interactive fuzzy selector with TUI ([dcde90a](https://codeberg.org/gbrennon/senkai/commit/dcde90aead95ce3e6b3b53a047561b8930f071b3))
- Add WorkspaceResolver for path and branch resolution ([85775b6](https://codeberg.org/gbrennon/senkai/commit/85775b6240a4bc79f65d4dcf2139645a58c9afb7))
- Add CommandRunner trait with System and Fake runners ([2d0eca1](https://codeberg.org/gbrennon/senkai/commit/2d0eca12da3570ed8ec9b4eac25ba574b5b7374a))
- Add GitExecutor with Box<dyn CommandRunner> injection ([e75433f](https://codeberg.org/gbrennon/senkai/commit/e75433f4add0984aafbf8bca430a253cfb8bf4b4))
- Add TmuxExecutor with Box<dyn CommandRunner> injection ([2e7baf8](https://codeberg.org/gbrennon/senkai/commit/2e7baf86b03e2a982ea9bbdfe248792d45daee8e))
- Add run() and run_with_terminal() routing functions ([f62ee45](https://codeberg.org/gbrennon/senkai/commit/f62ee4523fb6caf208e8362d6b1d86d5018e9587))
- Add TmuxPort and GitPort port traits ([fe45887](https://codeberg.org/gbrennon/senkai/commit/fe45887dc3a1fb3471b602a75a4e585e7015247a))
- Implement TmuxPort for TmuxExecutor ([0d57671](https://codeberg.org/gbrennon/senkai/commit/0d57671921e751154b91792738cb7c154441b07e))
- Implement GitPort for GitExecutor ([27c7958](https://codeberg.org/gbrennon/senkai/commit/27c795855a9a891f509c04a98f2b4602850da394))
- Add app module with port-based function signatures ([480d4fe](https://codeberg.org/gbrennon/senkai/commit/480d4fe7467ef141c944eb66e674c9e67847b488))
- Add lightweight zero-dependency Error type ([86e5fb3](https://codeberg.org/gbrennon/senkai/commit/86e5fb350dba8b5bba80ce3dbf6af1efe5ac4980))
- Add SelectorRunner port trait ([e1335e4](https://codeberg.org/gbrennon/senkai/commit/e1335e4a5a35ff9929f3be3673aacff886a0eb89))
- Add RatatuiSelector adapter with TerminalBackend ([77592f8](https://codeberg.org/gbrennon/senkai/commit/77592f8f8407ca47594a2e83b6cb7ec9c5f3ee3a))
- Add loading indicator with braille throbber animation ([61199ad](https://codeberg.org/gbrennon/senkai/commit/61199ad7bf0475e20a0c26e070b1b8030e905497))
- Add demo-loading command variant ([ab76bb9](https://codeberg.org/gbrennon/senkai/commit/ab76bb9903e28c844f542f730b42c8124999db10))
- Wire demo-loading dispatch and 3-second demo ([5dcebe1](https://codeberg.org/gbrennon/senkai/commit/5dcebe1ff19325546413fd55ed7fe8ffd491fe16))


### Changed
- Delegate tmux window kill to remove_worktree ([34b3d70](https://codeberg.org/gbrennon/senkai/commit/34b3d707033340cb9e997bed814fccd4c1c310ab))
- Restructure main to use core/infrastructure layers ([9bfeb41](https://codeberg.org/gbrennon/senkai/commit/9bfeb41690d7d8e598f24f6bb79c1d9d133fc05f))
- Remove old flat modules, migrate to core/infrastructure ([b27320d](https://codeberg.org/gbrennon/senkai/commit/b27320dcbe735cb63f9c87ad7476ff2daf020fe4))
- Refactor(): add pub mod pipeline and infrastructure exports ([6d4b068](https://codeberg.org/gbrennon/senkai/commit/6d4b0680971763ae40b222d0793f93c203c2fc9a))
- Refactor(): make shell_quote pub for pipeline extraction ([54cdfcc](https://codeberg.org/gbrennon/senkai/commit/54cdfcce47df16bf7dc48ad0fb9eed8b059c4c58))
- Export merge_checker and workspace_resolver ([6eea42e](https://codeberg.org/gbrennon/senkai/commit/6eea42e96c0f815aa368fdc35618cfb9ef327085))
- Export command_runner module ([a4f278a](https://codeberg.org/gbrennon/senkai/commit/a4f278a3fbca7228fb6246b711a0b19339beafe1))
- Refactor(): extract all pipeline logic from main.rs into Ctx-based module ([b658982](https://codeberg.org/gbrennon/senkai/commit/b65898269733c8e897188067b72db0b9a2775f2f))
- Refactor(): extract pipeline logic to pipeline module, keep main thin ([3fe8761](https://codeberg.org/gbrennon/senkai/commit/3fe876104e2afc5ecde9b37ace3099a8a4384425))
- Refactor(): thin main.rs to 5-line entry point

Extract all routing logic (is_terminal check, spawn_in_popup fallback,
error display) into pipeline::run() so it's testable and covered.
main.rs now only collects args, builds Ctx, and delegates. ([235e974](https://codeberg.org/gbrennon/senkai/commit/235e9746bd5b8b5c338ed7838798622d025d95a8))
- Wire app module, delete pipeline.rs ([249a660](https://codeberg.org/gbrennon/senkai/commit/249a660e7e5b7454b8b53e4083c6f967afd40cd0))
- Migrate tests to port-based app module ([b173661](https://codeberg.org/gbrennon/senkai/commit/b173661db27a1b736778f239e139b65682e2ab7f))
- Inline free functions into Selector impl, remove // comments ([a789b31](https://codeberg.org/gbrennon/senkai/commit/a789b31a210dd3aa24a99ba2c095a5b4acb3f7b2))
- Add Clone derive to FakeRunner, remove // section markers ([6dfa46a](https://codeberg.org/gbrennon/senkai/commit/6dfa46a1481a417ef34e077802df00fd27b60299))
- Use ShellQuoter, remove inline shell_quote, remove // comments ([4092705](https://codeberg.org/gbrennon/senkai/commit/4092705a874def292ab4ce52c59a334bdc1e3b9c))
- Replace app module with presentation module ([8c21df9](https://codeberg.org/gbrennon/senkai/commit/8c21df97dd3679393412cfc72c092f64c70c0a9e))
- Instantiate Cli struct and call run() ([0f3c785](https://codeberg.org/gbrennon/senkai/commit/0f3c7853eb58125ef61d04367c3a9ced4992732f))
- Replace free shell_quote with ShellQuoter struct ([f844ff9](https://codeberg.org/gbrennon/senkai/commit/f844ff9528146735a1648918e44996d221205b2b))
- Delete app.rs, fully replaced by presentation layer ([c92577e](https://codeberg.org/gbrennon/senkai/commit/c92577e6ccc408c6395716a6a8c4941857e3a510))
- Add DI container for production adapter wiring ([3d679fa](https://codeberg.org/gbrennon/senkai/commit/3d679fabd1a9e1d13bc1fad29bd2f33eb91ee0e3))
- Add composition root build_cli function ([09417f3](https://codeberg.org/gbrennon/senkai/commit/09417f3d0f3f0f124170761b8a3603644c656a58))
- Delegate adapter wiring to lib composition root ([0205164](https://codeberg.org/gbrennon/senkai/commit/02051642737c113acfcfaaa4ebee6871366c86e6))
- Dispatch on Command enum instead of raw strings ([236f8f4](https://codeberg.org/gbrennon/senkai/commit/236f8f4718f72d50a3f203b7c4e687dda0fe362a))
- Migrate port signatures from anyhow::Result to crate::core::error::Result ([99a1535](https://codeberg.org/gbrennon/senkai/commit/99a1535d1d089a98293ba6a4b4bc82a9830e4c57))
- Replace anyhow::Result with crate::core::error::Result in project_locator ([edf3658](https://codeberg.org/gbrennon/senkai/commit/edf365802f2c57b82bd3cc1aef369f8ba09f988b))
- Replace bail! and context() with Error::new in cli ([f587016](https://codeberg.org/gbrennon/senkai/commit/f587016df883e9163075e1fb942813c5c25399a5))
- Extract inline tests, replace anyhow! with Error::new in command ([c6e35d0](https://codeberg.org/gbrennon/senkai/commit/c6e35d078db8259d96e6088dda1b40ee5fe93df2))
- Extract find_repo_root_from to accept explicit start path ([3eda886](https://codeberg.org/gbrennon/senkai/commit/3eda88601d110eb3bc83982aa6076bc51459ce8c))
- Wire Cli to use SelectorRunner port ([61d3aaa](https://codeberg.org/gbrennon/senkai/commit/61d3aaaa292aca9b40ce37981f0085c2ee593da9))
- Declare selector modules ([3c4ca5b](https://codeberg.org/gbrennon/senkai/commit/3c4ca5bedfd50a04f7cc296aefa36509660f5ebe))
- Wire RatatuiSelector into Container ([ca23301](https://codeberg.org/gbrennon/senkai/commit/ca23301ed23fc660f6b6f5535ce7019a9d602678))
- Inline free functions into ProjectLocator impl ([1e1669f](https://codeberg.org/gbrennon/senkai/commit/1e1669fe0f9397e227aac1aea64c128b6d9695dc))
- Purify Selector with Key enum and substring matching ([d3fc832](https://codeberg.org/gbrennon/senkai/commit/d3fc832a94319d77a11b62e935283f4696edebe6))
- Add From<KeyEvent> impl and import style cleanup ([970ad0c](https://codeberg.org/gbrennon/senkai/commit/970ad0c39baa01ccfafd4bbd68e271b7fff35d27))
- Extract test_support module from command_runner ([8193e58](https://codeberg.org/gbrennon/senkai/commit/8193e58f667af25a25a297fcf82821012373ab45))
- Extract inline test from container ([1bcf328](https://codeberg.org/gbrennon/senkai/commit/1bcf3280858a34ad0380bbf8b07199847a322471))
- Extract inline tests from git_executor ([38c976d](https://codeberg.org/gbrennon/senkai/commit/38c976d36743a29da62527d41041be5ced7b9bcd))
- Extract inline tests from tmux_executor and pub window_exists ([dced2fc](https://codeberg.org/gbrennon/senkai/commit/dced2fc39b11bac410b506b796bdc99a5b3d7baf))
- Introducing modular functions ([7f3c7cf](https://codeberg.org/gbrennon/senkai/commit/7f3c7cf80f5165e662d8018fe8de28c950718cf6))
- Introduce download pre-built binary ([9bbabaf](https://codeberg.org/gbrennon/senkai/commit/9bbabaf97868dc11ef4add5c66f84b217dfd3184))
- Remove enable_strict_mode ([ddf1fd0](https://codeberg.org/gbrennon/senkai/commit/ddf1fd0bd56ac37f4ffcdaabbd13781d49a730cd))
- Add is_git_repo private method ([793e12f](https://codeberg.org/gbrennon/senkai/commit/793e12ffa6a62ef0fa8804fb0b04aadb6c9e6f0a))


### Documentation
- Replace make references with just after Makefile removal ([8614d8c](https://codeberg.org/gbrennon/senkai/commit/8614d8cb3030382405f09da5e677562de07835f7))
- Describe external worktree resume ([f653445](https://codeberg.org/gbrennon/senkai/commit/f6534457bb5b650fc850752c2e8dc284477c35c0))
- Clarify creation directory ([384d7ff](https://codeberg.org/gbrennon/senkai/commit/384d7ffcb8709963a77621af8606d465c1b7e74b))
- Link release documentation ([84ec009](https://codeberg.org/gbrennon/senkai/commit/84ec00952e6d831b1a438319a02cab78cf287ba4))
- Add changelog ([ecc805b](https://codeberg.org/gbrennon/senkai/commit/ecc805b1bbfece5fdea5b9840ba74e64990bc2d5))
- Document release process ([c662ea9](https://codeberg.org/gbrennon/senkai/commit/c662ea983cf58afb0b65d42e301814b7659ecde6))
- Document Codeberg branch pushes ([faebb91](https://codeberg.org/gbrennon/senkai/commit/faebb91f01a121b2692777ef6e2ccf546cda2333))
- Document external worktree management ([61a1155](https://codeberg.org/gbrennon/senkai/commit/61a115542d940757c11b984197e12f6e5f35fb13))
- Describe worktree picker labels ([7e18aec](https://codeberg.org/gbrennon/senkai/commit/7e18aece8c784f3a8da71172dbbcfe9d8ea31440))
- Align usage instructions with application ([49f57b6](https://codeberg.org/gbrennon/senkai/commit/49f57b6891462c2469fe5ffdbd293a0094f8eab5))
- Correct runtime prerequisites ([e7d3c4e](https://codeberg.org/gbrennon/senkai/commit/e7d3c4e10c4f39f8e644156dd868e5e52d15ef1d))
- Clarify repository root discovery ([f4fc0c7](https://codeberg.org/gbrennon/senkai/commit/f4fc0c7a1d5daebd2fe46b1c41a9cf1b4f890a6e))
- Clarify discovery and cleanup behavior ([f61aa25](https://codeberg.org/gbrennon/senkai/commit/f61aa25931a2ee6d234f742e434cdc991547d7da))
- Clarify cleanup ancestry checks ([57b9fd6](https://codeberg.org/gbrennon/senkai/commit/57b9fd6e18513904c323a6a729470c6389e91296))
- Normalize v0.3.0 changelog ([344140e](https://codeberg.org/gbrennon/senkai/commit/344140e9e7d9cb84d68bcf9aca3d09cf4b4458d9))


### Fixed
- Verify remote ref construction with RefCell capture ([414d551](https://codeberg.org/gbrennon/senkai/commit/414d5512dd472f0ff8a5904705e20b66de15f2f5))
- Map merge-base exit status to bool in is_merged closure ([821541e](https://codeberg.org/gbrennon/senkai/commit/821541ed29cb6ffb908a0edeb20968777a6035b5))
- Wrap unsafe env::set_var in app entry point ([915ed38](https://codeberg.org/gbrennon/senkai/commit/915ed3819b2a3978b7f7d6060d4e2de4ce347196))
- Wrap unsafe env calls in project_locator tests ([a8d86e1](https://codeberg.org/gbrennon/senkai/commit/a8d86e1e4b4bd6011a463e5b899cfd1d850d97bc))
- Wrap unsafe env calls in tmux_executor tests ([d437ade](https://codeberg.org/gbrennon/senkai/commit/d437adea04be32281a0fb59427c27445d133e647))
- Wrap unsafe env calls in e2e worktree_lifecycle tests ([8ac5431](https://codeberg.org/gbrennon/senkai/commit/8ac54310a1ad8ff977b2ec8b63fbfd8734de4037))
- Install all dependencies in install-dependencies action ([3052fc2](https://codeberg.org/gbrennon/senkai/commit/3052fc20dee7da595e70c5a6b818273a44e8fd43))
- Make workflows portable across act and forgejo-runner ([9256f5c](https://codeberg.org/gbrennon/senkai/commit/9256f5c7e95eb2eee861585f616dd6019d923475))
- Add missing IsTerminal import ([25f1354](https://codeberg.org/gbrennon/senkai/commit/25f1354273b7901851f00fb21a40685309c79d16))
- Handle current_exe failure gracefully in spawn_in_popup ([b83f9d7](https://codeberg.org/gbrennon/senkai/commit/b83f9d7ce7a3c403425784e64a87f6165336ed72))
- Serialize env-var-dependent e2e tests with Mutex, clean up leaks ([9a7342e](https://codeberg.org/gbrennon/senkai/commit/9a7342e3cd8cb01fea082ec4b31c6f1cf12e0579))
- Remove redundant modifiers: _ pattern for clippy 1.97 ([0d0d200](https://codeberg.org/gbrennon/senkai/commit/0d0d200ebbcd242ab49696bb44561ac983432223))
- Use __ROOT__ template in FakeGitForCleanup for CI portability ([486e748](https://codeberg.org/gbrennon/senkai/commit/486e74884b95112e03ad55013b86e08b7df43193))
- Enforce minimum display time for loading indicator ([8d7f96e](https://codeberg.org/gbrennon/senkai/commit/8d7f96ee8c1343632f94b4d5960111e429100e6e))
- Open renamed worktrees by path ([9b51b7b](https://codeberg.org/gbrennon/senkai/commit/9b51b7b73e31b5cbd5369495693a34bb27ff5c7d))
- Retain discovered worktree paths ([18e5f78](https://codeberg.org/gbrennon/senkai/commit/18e5f782a4c89139796a58e4507dedbb220f27b0))
- Discover worktrees from git ([0e73b7c](https://codeberg.org/gbrennon/senkai/commit/0e73b7cf67ab17fa29407feecd7321c093d737d0))
- Remove unused e2e import ([dcf6f96](https://codeberg.org/gbrennon/senkai/commit/dcf6f9657e0a64fed5b4a1a168ba9101afdff39b))
- Reap isolated tmux server at process exit ([284580c](https://codeberg.org/gbrennon/senkai/commit/284580ce50c6a43f54e930cb69c49aeaea08cb31))


### Maintenance
- Ignore rust build target dir ([e39a500](https://codeberg.org/gbrennon/senkai/commit/e39a5008c33cc2fd3459b86f3f98dafd3b489cb7))
- Keep Cargo.lock tracked for reproducible builds ([105d9a4](https://codeberg.org/gbrennon/senkai/commit/105d9a46d8dd7a3a885a8bb12396d2ddd2c6c84d))
- Add rust crate manifest ([0bc35f9](https://codeberg.org/gbrennon/senkai/commit/0bc35f9e250a7118671c7b0878806a33bc151290))
- Add install script for tpm/tpack ([b371c33](https://codeberg.org/gbrennon/senkai/commit/b371c33a830ef5314b1b19a25ab72d91034fcb0c))
- Rm verification shell script ([b43e882](https://codeberg.org/gbrennon/senkai/commit/b43e882d7ba7cd439165ae0d30c79310e1916869))
- Chore(): add test, coverage, fmt, lint, and tool recipes ([cfe3433](https://codeberg.org/gbrennon/senkai/commit/cfe3433ffcc9171f540bc06c251cd40af6dd7541))
- Chore(): add ratatui dep, coverage config, test entries ([36dcec5](https://codeberg.org/gbrennon/senkai/commit/36dcec5d9c9990e7640b1aea1098d8fe872f71aa))
- Rm legacy files ([dd43b4f](https://codeberg.org/gbrennon/senkai/commit/dd43b4ff7e60dd9248cbb42b1a66721004ef6e82))
- Wip ci/cd ([a20d099](https://codeberg.org/gbrennon/senkai/commit/a20d09904602fe751888ccbf0b886e9e08a65194))
- Rm gitmodules ([ea21569](https://codeberg.org/gbrennon/senkai/commit/ea215691854de3037516032896113f1f55900f36))
- Remove Makefile ([ec953ed](https://codeberg.org/gbrennon/senkai/commit/ec953edcda1abd77c56761165cfe856f6ddb7d5f))
- Remove make from install-dependencies action ([8cb2ca5](https://codeberg.org/gbrennon/senkai/commit/8cb2ca5dc0c245d017459d0dbf96f1ab24d2482e))
- Add lefthook pre-commit config for fmt and lint ([00decda](https://codeberg.org/gbrennon/senkai/commit/00decda89373a0717a97050abb67ae465fe878f8))
- Implement real test, lint, and validate jobs in CI workflow ([d3ce0e5](https://codeberg.org/gbrennon/senkai/commit/d3ce0e54a8f066f105c55c30afaca56b32874c91))
- Remove redundant test workflow covered by ci.yml ([9663721](https://codeberg.org/gbrennon/senkai/commit/9663721f3b72b8e90ed5c2925aea325a31803c07))
- Replace wip placeholder with just test in run-tests action ([6b3ced3](https://codeberg.org/gbrennon/senkai/commit/6b3ced3570c36c025e4633d78bc97919436fa4da))
- Add git helper functions for CI checks ([1c0eb39](https://codeberg.org/gbrennon/senkai/commit/1c0eb39796f5f1caab18463bd927dc2d1c9475e6))
- Add conventional commit validation script ([301e398](https://codeberg.org/gbrennon/senkai/commit/301e398e86bf7e9a677ebf03798d6f08270a3d9d))
- Add branch name validation script ([8636817](https://codeberg.org/gbrennon/senkai/commit/8636817e6d67fb7cf1e9fd775c62659d0879fee9))
- Add commit message validation workflow ([b11ba72](https://codeberg.org/gbrennon/senkai/commit/b11ba7240253560f9f58e468f792fef7f32e03ad))
- Add branch name validation workflow ([1398f97](https://codeberg.org/gbrennon/senkai/commit/1398f97ba32d2e819b3cc4d689c6ec6a661673e5))
- Add actionlint configuration ([ab1f7b9](https://codeberg.org/gbrennon/senkai/commit/ab1f7b951c71b9d694d4a9c07fad0c7595335a0d))
- Add lint-workflows recipe to justfile ([28d72e8](https://codeberg.org/gbrennon/senkai/commit/28d72e87abb944ad8b4a1263ffdb7827242c9ba5))
- Bump Rust edition to 2024 ([28ad49c](https://codeberg.org/gbrennon/senkai/commit/28ad49c396bc24d4198e7dbd106d1eb6994155e5))
- Inline setup-rust steps, drop custom composite action ([50680f3](https://codeberg.org/gbrennon/senkai/commit/50680f3941277688ae4dde6ebe3134c2d0ca41a9))
- Remove anyhow dependency, add presentation test targets ([c2f73ac](https://codeberg.org/gbrennon/senkai/commit/c2f73ac461aac00118cb1641ea8d27f045946606))
- Suppress dead_code warnings in shared fakes ([6a136ef](https://codeberg.org/gbrennon/senkai/commit/6a136ef53e2ba736dff44c1860140e259c583b74))
- Restructure common.sh ([c85533e](https://codeberg.org/gbrennon/senkai/commit/c85533e9048586b2ee839f237b52312c33b8e9b4))
- Add branch name guard ([b01791e](https://codeberg.org/gbrennon/senkai/commit/b01791e9424b0ba14822072cf04750fe756878b6))
- Pull release refactor ([529b517](https://codeberg.org/gbrennon/senkai/commit/529b517000b1f7f9d725db1e070121518ce3528a))
- Add cache tp ci.yml ([7d4bcc7](https://codeberg.org/gbrennon/senkai/commit/7d4bcc75d61b74de0c31dc18c4813755b59515bb))
- Change codeberg size runner ([af75063](https://codeberg.org/gbrennon/senkai/commit/af750633811a28f9462206212c461a906a274797))
- Configure git cliff ([2120361](https://codeberg.org/gbrennon/senkai/commit/212036104a147a3b3946b72bf3a05dcef37f44ad))
- Configure Forgejo permissions ([5e67a4d](https://codeberg.org/gbrennon/senkai/commit/5e67a4dacca56046fafee5ef0eaf43842e9becbc))
- Serialize coverage test execution ([aefde66](https://codeberg.org/gbrennon/senkai/commit/aefde66c6532c16db67bbb14376d008adba5f5d9))
- Recognize legacy tags ([4ad38c3](https://codeberg.org/gbrennon/senkai/commit/4ad38c36ec89191584c53936cca5a1612f5db432))
- Add crate package information ([4f93614](https://codeberg.org/gbrennon/senkai/commit/4f93614e0218d6b27830b4b68dd166b1238def15))
- Prepare v0.3.0 ([c1ad78c](https://codeberg.org/gbrennon/senkai/commit/c1ad78cce186c7fe26fbcba7610cd135b929f191))
- Update changelog ([1c20267](https://codeberg.org/gbrennon/senkai/commit/1c20267f1a18094efc64d754fde39cd0e7b06db6))
- Update changelog ([4218361](https://codeberg.org/gbrennon/senkai/commit/421836143802d93f56f23f15c76e8cd067fbea8f))


### Other
- Merge pull request 'fix/remove-worktree' (#15) from fix/remove-worktree into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/15 ([4c7235b](https://codeberg.org/gbrennon/senkai/commit/4c7235b8c9455383d778d37435c3e9a92bec6ccd))
- Merge pull request 'feat/rust-port' (#16) from feat/rust-port into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/16 ([0be47b5](https://codeberg.org/gbrennon/senkai/commit/0be47b54e3092d5135fb41244170b94579d566c4))
- Merge branch 'main' of ssh://codeberg.org/gbrennon/tmux-worktrees ([55c456b](https://codeberg.org/gbrennon/senkai/commit/55c456b7a2e380967104dd77a381a503f54e776e))
- Add lib target and dev dependencies ([4a8f291](https://codeberg.org/gbrennon/senkai/commit/4a8f29105503ede40bd1d2bda3d2638bed523bb7))
- Revert "test(pipeline): add FakeRunner-backed test for run_with_terminal dispatch"

This reverts commit b554bd8fc466d7593730d4e696643d202beea4b0. ([33f57fc](https://codeberg.org/gbrennon/senkai/commit/33f57fcb179d50cbab07a023febc0dcc07bae309))
- Revert "feat(pipeline): add run() and run_with_terminal() routing functions"

This reverts commit f62ee4523fb6caf208e8362d6b1d86d5018e9587. ([13a786f](https://codeberg.org/gbrennon/senkai/commit/13a786f48085104822b6d38362e93d2e4be26518))
- Revert "refactor(): thin main.rs to 5-line entry point"

This reverts commit 235e9746bd5b8b5c338ed7838798622d025d95a8. ([47dc57a](https://codeberg.org/gbrennon/senkai/commit/47dc57aba11915ef601a8404b1023f6ef2122427))
- Merge pull request 'feat/tmux-worktrees-rust' (#17) from feat/tmux-worktrees-rust into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/17
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([9d5a1a4](https://codeberg.org/gbrennon/senkai/commit/9d5a1a49f5e76cfb0b5e151e99381d6c49772271))
- Merge branch 'main' of ssh://codeberg.org/gbrennon/tmux-worktrees ([11cc57b](https://codeberg.org/gbrennon/senkai/commit/11cc57bda7b3ce91dd7d117b9642f2484d22270e))
- Merge pull request 'chore: rm legacy files' (#18) from chore/rm-legacy-files into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/18 ([4b446f2](https://codeberg.org/gbrennon/senkai/commit/4b446f2fd99decd5b0b1682e96f3a3b65cde2bb7))
- Install just in install-dependencies action ([61f1221](https://codeberg.org/gbrennon/senkai/commit/61f1221fa705501f51de8fbe81f6cbbe326ced10))
- Apply rustfmt formatting to source files ([8cb5f91](https://codeberg.org/gbrennon/senkai/commit/8cb5f912d6bb489003b6ac8e6eadab9eb46fe842))
- Merge pull request 'chore/forgejo-rust-pipeline' (#19) from chore/forgejo-rust-pipeline into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/19 ([a216356](https://codeberg.org/gbrennon/senkai/commit/a21635685255e5d2573acfd2b0668ea5e4e93b3d))
- Merge branch 'main' of ssh://codeberg.org/gbrennon/tmux-worktrees ([6a41a9b](https://codeberg.org/gbrennon/senkai/commit/6a41a9b064d1558d5dd5f4dd16e90c8d7389c44e))
- Remove // comments from test module ([7e8acea](https://codeberg.org/gbrennon/senkai/commit/7e8aceaac76af8a75bce5a9ec19dfd128a68cf5f))
- Remove // comments from test module ([762edbd](https://codeberg.org/gbrennon/senkai/commit/762edbd8681d6e5d41688410b5d28081efc9865b))
- Remove // section markers from test module ([463b7a1](https://codeberg.org/gbrennon/senkai/commit/463b7a1c38c384d450da2f895cb8e1e5b83c055e))
- Remove // section markers ([7ca9bb4](https://codeberg.org/gbrennon/senkai/commit/7ca9bb4e060121a470ecb1f0b1976b46e971da56))
- Normalize vec formatting and remove blank lines ([917ec54](https://codeberg.org/gbrennon/senkai/commit/917ec54629dde36c5081a7317583e6abc4d29364))
- Apply rustfmt to container.rs ([ed5d630](https://codeberg.org/gbrennon/senkai/commit/ed5d630273db39b7e42435874a4bff76ae0173e6))
- Fix brace placement on setup_terminal signature ([8e742fc](https://codeberg.org/gbrennon/senkai/commit/8e742fc1459f278722fdd5cced2b78dda5be4856))
- Order mod declarations alphabetically ([8cac24d](https://codeberg.org/gbrennon/senkai/commit/8cac24d98b6ce98e705214785b0d2d5ccff500a0))
- Merge pull request 'chore/repo-hygiene' (#20) from chore/repo-hygiene into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/20
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([2598605](https://codeberg.org/gbrennon/senkai/commit/2598605fffa90ca9939b040ca40c2ac2efbe7835))
- Merge branch 'main' of ssh://codeberg.org/gbrennon/tmux-worktrees ([441e0b5](https://codeberg.org/gbrennon/senkai/commit/441e0b50fafaa7257b470a7e3e9f1b9318ee274c))
- Merge pull request 'feat/loading-indicator' (#21) from feat/loading-indicator into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/21 ([9b95128](https://codeberg.org/gbrennon/senkai/commit/9b95128801fb1d435f46e0072e51beb0cc56f443))
- Merge branch 'main' of ssh://codeberg.org/gbrennon/tmux-worktrees ([9243f85](https://codeberg.org/gbrennon/senkai/commit/9243f85d9c2348446a78c0bf0974bd034a11720a))
- Merge branch 'fix/loading' ([7811065](https://codeberg.org/gbrennon/senkai/commit/7811065e861be70c57e334b7f3f3182e0c845309))
- Remove comments ([55878c4](https://codeberg.org/gbrennon/senkai/commit/55878c45ee5f6003b339e0b5ecd208dba8bea541))
- Merge pull request 'fix/loading' (#22) from fix/loading into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/22
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([fbf79f3](https://codeberg.org/gbrennon/senkai/commit/fbf79f30cde6a07f273e74c3ec42e983b87b4fdb))
- Merge remote-tracking branch 'origin/main' ([727c948](https://codeberg.org/gbrennon/senkai/commit/727c94866f3e53cf9f1b6147164e955130715a06))
- Merge pull request 'feat/tmux-worktree-refactor' (#23) from feat/tmux-worktree-refactor into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/23
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([49f2c6b](https://codeberg.org/gbrennon/senkai/commit/49f2c6ba5aee7efd709548b0b0d66c3acd1cec71))
- Merge branch 'main' of ssh://codeberg.org/gbrennon/tmux-worktrees ([c70aec3](https://codeberg.org/gbrennon/senkai/commit/c70aec3f2d8c2f428fa568242bedede6f319d283))
- Add whiteline ([b949398](https://codeberg.org/gbrennon/senkai/commit/b94939884807f3bd296dc7d5f16d7b42d13660b5))
- Merge pull request 'fix(cli): open renamed worktrees by path' (#24) from fix/open-renamed-worktrees into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/24
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([a9cd1f8](https://codeberg.org/gbrennon/senkai/commit/a9cd1f8ea04bb18275b83e236d1a82ddddc7cf52))
- Merge remote-tracking branch 'origin/main' ([e5cba93](https://codeberg.org/gbrennon/senkai/commit/e5cba937b1b050883c76a966c106f6da05aa76e8))
- Merge pull request 'fix: resume worktrees from git paths' (#25) from fix/resume-external-worktrees into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/25 ([c4f5cfc](https://codeberg.org/gbrennon/senkai/commit/c4f5cfcccb946eb0134673f4976398a3936f9737))
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([469ea73](https://codeberg.org/gbrennon/senkai/commit/469ea7369c41c23814f409db882b5c60e50e8ff0))
- Run release script tests ([c6812ad](https://codeberg.org/gbrennon/senkai/commit/c6812ada2e7a704caa41e73bb9b3fe720bb08f4f))
- Lint release scripts ([1064571](https://codeberg.org/gbrennon/senkai/commit/1064571a16be9f6160ad91712109c6f0b0814a3f))
- Add crate package metadata ([337d405](https://codeberg.org/gbrennon/senkai/commit/337d405fb440cf5c2b6406b55e55077f5ca98673))
- Add release creation action ([6be9cde](https://codeberg.org/gbrennon/senkai/commit/6be9cde491f2e10c0be6c6085647a42872763dbd))
- Add release notes action ([08f1fd8](https://codeberg.org/gbrennon/senkai/commit/08f1fd874558eaa428eec8c0f0d691392072e227))
- Add git cliff installer ([e5cd830](https://codeberg.org/gbrennon/senkai/commit/e5cd83044c69b34c637d5f37148d0701d47a0581))
- Add release tooling installer ([eef3381](https://codeberg.org/gbrennon/senkai/commit/eef3381e0220d89b45a633d1614b67c9e8a0a5c0))
- Add release preflight action ([37eb3eb](https://codeberg.org/gbrennon/senkai/commit/37eb3ebe0abd6fb487dccb8bd39b1548ac68b3bb))
- Add crate publish action ([f2723be](https://codeberg.org/gbrennon/senkai/commit/f2723bea6495ff69f8c4b4638201a8120a16dc35))
- Add release tag action ([773380c](https://codeberg.org/gbrennon/senkai/commit/773380c1e690ed8a8ba297a9c6fe68b06f1da73e))
- Add changelog action ([b797f7d](https://codeberg.org/gbrennon/senkai/commit/b797f7de10103320fb06e7a8443e08d18d0a4d56))
- Add release validation action ([ad3d5b1](https://codeberg.org/gbrennon/senkai/commit/ad3d5b18e9ee0de94320d9c8a2416a5a2c1f701e))
- Prepare release branches ([e8397ef](https://codeberg.org/gbrennon/senkai/commit/e8397ef6385f28497122ea9264a969dc7ea1945a))
- Publish releases ([8c2a2b9](https://codeberg.org/gbrennon/senkai/commit/8c2a2b97f8da543ad37dd2ef1847b12952ccc0ef))
- Add release API script ([7fbd285](https://codeberg.org/gbrennon/senkai/commit/7fbd285e1724d01c9cc591a8d7426f09683c8887))
- Add release notes script ([67c3ea1](https://codeberg.org/gbrennon/senkai/commit/67c3ea175d31d51f9b3e5bfefa0b7810561d1246))
- Add release preflight script ([fca98d4](https://codeberg.org/gbrennon/senkai/commit/fca98d4e543b7cd6133d2d12a054e1b3eb4ab7bf))
- Add crate publish script ([4197a40](https://codeberg.org/gbrennon/senkai/commit/4197a40c858c99efe9e4138a4c735bd7a5aab375))
- Add release tag script ([29c3a2b](https://codeberg.org/gbrennon/senkai/commit/29c3a2ba81cad6a156041d9e859bea6d7afb2dfe))
- Add changelog script ([b7b4f4f](https://codeberg.org/gbrennon/senkai/commit/b7b4f4ff5a224f67ef823ec88efd2f9c4d9aa086))
- Add release validation script ([6ee0374](https://codeberg.org/gbrennon/senkai/commit/6ee03746828fc1a439446068296a3632eb72c5c7))
- Tolerate unavailable cache service ([bfc5078](https://codeberg.org/gbrennon/senkai/commit/bfc5078dc795ff0191623d73456c541934275a23))
- Name test action steps ([89b8c44](https://codeberg.org/gbrennon/senkai/commit/89b8c441d575d1679345f81a696b040cf1405787))
- Remove unreliable cargo caches ([6f290d2](https://codeberg.org/gbrennon/senkai/commit/6f290d2e0d3ece55d7e672ca82b8ec74b4061d4e))
- Merge pull request 'ci: automate Cargo and GitHub releases' (#26) from feat/release-automation into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/26 ([e2eda2e](https://codeberg.org/gbrennon/senkai/commit/e2eda2eb8966b4517507bc5892761b1eb2616fcd))
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([858af85](https://codeberg.org/gbrennon/senkai/commit/858af851ef6a2e00fd83cfc6e254fb2a3cc61022))
- Merge pull request 'chore(ci): configure Forgejo permissions' (#28) from chore/forgejo-permissions into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/28⁩ ([135f2d6](https://codeberg.org/gbrennon/senkai/commit/135f2d6a9633feb688c32f3d1f96768575459ee7))
- Merge pull request 'chore(test): serialize coverage test execution' (#30) from chore/serialize-ci-tests into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/30 ([d83c26d](https://codeberg.org/gbrennon/senkai/commit/d83c26dd52268a51ae9efba5d6cca70dea3613b5))
- Install rust for release preparation ([2edf9ae](https://codeberg.org/gbrennon/senkai/commit/2edf9aee05db8f64bee72a7da5d9e42bc4ab2dc3))
- Install rust for publishing releases ([6d7d8d9](https://codeberg.org/gbrennon/senkai/commit/6d7d8d937399e9d5faae0a3e092739882090254a))
- Merge pull request 'chore(ci): prepare release automation for v0.3.0' (#31) from chore/release-readiness into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/31⁩ ([99464d5](https://codeberg.org/gbrennon/senkai/commit/99464d5aad99d31c6f8f003b12d1d27bff1c7a71))
- Merge branch 'main' of github.com:gbrennon/tmux-worktrees ([470f426](https://codeberg.org/gbrennon/senkai/commit/470f4269b8eff5843f99e97b92208d2bccf7d721))
- Merge pull request 'chore(docs): document external worktree management' (#32) from chore/document-external-worktrees into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/32⁩ ([76df6d9](https://codeberg.org/gbrennon/senkai/commit/76df6d9843ccc0a0f31b545b00d28408a40b720d))
- Verify release pull request checks ([eb80b33](https://codeberg.org/gbrennon/senkai/commit/eb80b33e28d070f80a3f02b94126791238209d3e))
- Retry release pull request checks ([fbc177d](https://codeberg.org/gbrennon/senkai/commit/fbc177ddab5e9a94742ade984a2d268a9051ed48))
- Merge pull request 'fix(tests): reap isolated tmux server at process exit' (#35) from fix/tmux-test-server-leak into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/35 ([654efcb](https://codeberg.org/gbrennon/senkai/commit/654efcb809a14662b1aa8d3ff28a947c123ea271))
- Merge branch 'main' into release/v0.3.0 ([ecefee8](https://codeberg.org/gbrennon/senkai/commit/ecefee80f6dfe6c96dfa50c2822ccd81f6d6d505))
- Merge pull request 'release: v0.3.0' (#33) from release/v0.3.0 into main

Reviewed-on: ⁨https://codeberg.org/gbrennon/tmux-worktrees/pulls/33⁩ ([5a71c76](https://codeberg.org/gbrennon/senkai/commit/5a71c76017676dfed3470c7d347aee5da5631481))


### Tests
- Verify remove_worktree calls tmux_kill_window ([79f58ae](https://codeberg.org/gbrennon/senkai/commit/79f58aeb689d655477b8731f585f906e6b1ef2d8))
- Add integration tests with fakes for git/tmux executors ([817114f](https://codeberg.org/gbrennon/senkai/commit/817114fe295d9e0c5d275684d58d0c27f8a918bc))
- Add FakeRunner-based unit tests for GitExecutor ([b391c46](https://codeberg.org/gbrennon/senkai/commit/b391c46556173cb2be590b06db2aed00bad6cbb9))
- Add isolated tmux server tests for TmuxExecutor ([8f35e4a](https://codeberg.org/gbrennon/senkai/commit/8f35e4a519fc27edd8e272beb48efc0f4ab1f039))
- Add end-to-end lifecycle tests with ephemeral repos ([6a9ae79](https://codeberg.org/gbrennon/senkai/commit/6a9ae793fd8a8fffd83b69653793f18ddaaba490))
- Add FakeRunner-backed test for run_with_terminal dispatch ([b554bd8](https://codeberg.org/gbrennon/senkai/commit/b554bd8fc466d7593730d4e696643d202beea4b0))
- Update to use Cli struct instead of deleted app module ([05a20d3](https://codeberg.org/gbrennon/senkai/commit/05a20d3fcc9b04d30a618145b3d950d434360551))
- Add as_str and InvalidCommand tests for command.rs ([c221ac8](https://codeberg.org/gbrennon/senkai/commit/c221ac814cf544c37aa08c5a318f81d710dfae8b))
- Update worktree lifecycle tests for Command enum refactor ([dede579](https://codeberg.org/gbrennon/senkai/commit/dede57962e94c2ff02fbd583924aba58c026a1db))
- Add unit tests ([8bb3037](https://codeberg.org/gbrennon/senkai/commit/8bb303762cecd86fff271556d1b2626ff8122451))
- Add create_workspace failure path coverage ([52cbf2c](https://codeberg.org/gbrennon/senkai/commit/52cbf2c9ea6b540e640789ab477e2bb22a26a567))
- Update git_executor test imports for new error type ([0ee86b9](https://codeberg.org/gbrennon/senkai/commit/0ee86b9546328a47a95377a68a0454020dcd7912))
- Update cli test imports and extend error-path coverage ([ed6ed84](https://codeberg.org/gbrennon/senkai/commit/ed6ed84e58e61b2a5c376c7d4037625f9d72d888))
- Implement fallback port methods on test fakes to prevent flaky panics ([39050b0](https://codeberg.org/gbrennon/senkai/commit/39050b060ac096b01a76f42b7c9148ae879ef165))
- Add FakeSelector and coverage tests for Cli ([d57468c](https://codeberg.org/gbrennon/senkai/commit/d57468c57cf04c1081952f31d39ac8a342fe64cb))
- Pass RatatuiSelector to e2e Cli::new ([47ffa9f](https://codeberg.org/gbrennon/senkai/commit/47ffa9f760cdb8151ddc558ed407c68ce08b5bd1))
- Add tests/common/mod.rs module scaffold ([8f75396](https://codeberg.org/gbrennon/senkai/commit/8f7539644f78b6f8343b584f3c8e81fa502c850e))
- Add tests/common/fakes/mod.rs submodule declarations ([4a59058](https://codeberg.org/gbrennon/senkai/commit/4a5905848c0d811af0fa6d22b114a392d9f6c2db))
- Extract TmuxPort fakes to tests/common/fakes/tmux.rs ([9fb213a](https://codeberg.org/gbrennon/senkai/commit/9fb213a4a41ecdd4aa955d74a9f30c67a3896188))
- Extract GitPort fakes to tests/common/fakes/git.rs ([ebf3a6b](https://codeberg.org/gbrennon/senkai/commit/ebf3a6b570d36433f544afdf86dc571859b69f1c))
- Extract SelectorRunner fakes to tests/common/fakes/selector.rs ([23f65b6](https://codeberg.org/gbrennon/senkai/commit/23f65b6a3b978b43fd53136773221526b47fe610))
- Import extracted fakes from tests/common/fakes in cli_test ([eb77456](https://codeberg.org/gbrennon/senkai/commit/eb774560ff3a6d0f181ce11a36cda23f759d217d))
- Add FakeRunner shared test fake ([baadbc1](https://codeberg.org/gbrennon/senkai/commit/baadbc16aa96cca154230199fc54a8ce259776fa))
- Register command_runner module ([d100554](https://codeberg.org/gbrennon/senkai/commit/d10055429a29e71630915d97dd4b97561f84d2f7))
- Add FakeRunner-based integration tests for git executor ([63e2c48](https://codeberg.org/gbrennon/senkai/commit/63e2c486b26acbfbb105b6eaf82ca8ecb458554c))
- Add FakeRunner-based integration tests for tmux executor ([b11cfc5](https://codeberg.org/gbrennon/senkai/commit/b11cfc523a49609c619b0aa53622e85f896a05c5))
- Add integration test for container bootstrap ([40577a9](https://codeberg.org/gbrennon/senkai/commit/40577a97002cdb6d2e570ff7e3acccd0faae0640))
- Add FakeLoading and loading indicator tests ([bda7735](https://codeberg.org/gbrennon/senkai/commit/bda77353dd6ae62e5c72e5acaad7875726a4ceb1))
- Rename worktree lifecycle test to *_test ([4825084](https://codeberg.org/gbrennon/senkai/commit/482508430a22c82f518974f8dd1e34a82cc67516))
- Group executor tests under single target ([c92fad4](https://codeberg.org/gbrennon/senkai/commit/c92fad4e7520d6ef83f5a009d6e9642585a54865))
- Group cli and command tests under single target ([1ce28d2](https://codeberg.org/gbrennon/senkai/commit/1ce28d2b61530798d5fc3e659c8abbbd69ecfbc7))
- Route lifecycle test through module root ([0080fa2](https://codeberg.org/gbrennon/senkai/commit/0080fa23d132aa11c9eaf2fbcf42d2165354fb85))
- Scaffold test module mirroring src/core ([59090ed](https://codeberg.org/gbrennon/senkai/commit/59090ed66ed210f0ab97baebd0907b421c56f18d))
- Support renamed worktree fixtures ([7cfa39d](https://codeberg.org/gbrennon/senkai/commit/7cfa39d843cb61bc66c9c0bf5b4adb85b69b9ad8))
- Record selected workspace window ([0eb997a](https://codeberg.org/gbrennon/senkai/commit/0eb997a8e2e089b4fcc07a4120b49af2a40c39de))
- Cover renamed worktree opening ([b074737](https://codeberg.org/gbrennon/senkai/commit/b0747377af5f416ef2290375eacc3dd61f60b949))
- Cover external worktree paths ([cca58ce](https://codeberg.org/gbrennon/senkai/commit/cca58ce5a7ec28f70174b75ae2cb0fe792d8a990))
- Resume external worktree ([ade2d1c](https://codeberg.org/gbrennon/senkai/commit/ade2d1c681571670fec8954a032b6e166611dc48))
- Register external worktree test ([d1fa084](https://codeberg.org/gbrennon/senkai/commit/d1fa084fc5a532fe398e1ff017395f763de428bb))
- Update worktree listing fixtures ([abc9ac9](https://codeberg.org/gbrennon/senkai/commit/abc9ac9c69b7d6765eb7f1a671dad1abde0903e0))
- Cover external resume paths ([48e391e](https://codeberg.org/gbrennon/senkai/commit/48e391ee6935ee3f53d81987c29ef389cdb8cf96))
- Add release script recipe ([878772e](https://codeberg.org/gbrennon/senkai/commit/878772e77b2bf381651be39e88e7afbd5f281042))
- Cover release scripts ([f0e395f](https://codeberg.org/gbrennon/senkai/commit/f0e395ff2e79ca5fafb670ed474b4b63bd1c7eda))

## [0.3] - 2026-06-16


### Fixed
- Worktree.sh bug ([fa986d4](https://codeberg.org/gbrennon/senkai/commit/fa986d4221bfda0e3fbc808d16b6e08951fcc9a5))


### Maintenance
- Adjust if clause ([f38da09](https://codeberg.org/gbrennon/senkai/commit/f38da09a42b654c6ca17069b5d5d8c0b24de2bd4))
- Fix makefile ([d33bc1b](https://codeberg.org/gbrennon/senkai/commit/d33bc1b6c39dc0270f6715998304d180caf573ba))
- Fix makefile ([818bcc9](https://codeberg.org/gbrennon/senkai/commit/818bcc9cafb60501b091b5448387b787105fca81))


### Other
- Merge branch 'main' into fix/git-worktree-unshallow ([d2ed7a3](https://codeberg.org/gbrennon/senkai/commit/d2ed7a3d2243fb9f9c5f398cce8102d7fbf58311))
- Merge pull request 'fix(lib/merge): git fetch the repo completely' (#12) from fix/git-worktree-unshallow into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/12
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([0fde6b6](https://codeberg.org/gbrennon/senkai/commit/0fde6b6c3318caf4e91256bfad82ad0485bd9f0a))
- Merge branch 'main' into fix/create-worktree ([86f53fd](https://codeberg.org/gbrennon/senkai/commit/86f53fd7c9558b1b7f6a3f7c9908cc383865b5c7))
- Merge pull request 'refactor(worktree): deep find' (#13) from fix/create-worktree into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/13 ([035972b](https://codeberg.org/gbrennon/senkai/commit/035972be250ccc0c5cb0ec8caa5ccc3a3aff25ca))
- Merge pull request 'chore(Makefile): adjust if clause' (#14) from chore/makefile-fix into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktrees/pulls/14
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([a238bf6](https://codeberg.org/gbrennon/senkai/commit/a238bf65827ae9e6344fc3f417f2b0d55815a517))

## [0.2] - 2026-05-25


### Added
- Manager supports custom dir ([e2cd763](https://codeberg.org/gbrennon/senkai/commit/e2cd763b4f413b481364a5c75775a67da8106c1d))


### Changed
- Deep find ([31f9144](https://codeberg.org/gbrennon/senkai/commit/31f91449ea54e687d230fab390c5b89408d64df4))


### Documentation
- New docs to improve knowledge sharing ([327bb17](https://codeberg.org/gbrennon/senkai/commit/327bb172ae02d7beaff4934402659cac285392bb))
- Contributors.md written ([1e453c1](https://codeberg.org/gbrennon/senkai/commit/1e453c141326e605a8aa45ee27116ee56ef41cd1))
- LICENSE file created ([4d8d154](https://codeberg.org/gbrennon/senkai/commit/4d8d154f8a2b94cbd6cf71c93f5b57c15c7f77a7))


### Fixed
- Git fetch the repo completely ([144e6f0](https://codeberg.org/gbrennon/senkai/commit/144e6f01570c0e04b2e2b4a88f7639fc891050ab))


### Maintenance
- Add KCOV to show test coverage report ([9d9242d](https://codeberg.org/gbrennon/senkai/commit/9d9242de4e8191993f61d0694cd73b516c913dfc))
- Fix in test coverage report ([b1c8e10](https://codeberg.org/gbrennon/senkai/commit/b1c8e10fc418e0c7c8a5d9bf1fbaf3ce1be88795))
- Fix test make command ([9d8a453](https://codeberg.org/gbrennon/senkai/commit/9d8a453e21051a5dcc0452ed2d2618d56fb1fbab))


### Other
- Style(worktree-manager): ([381a7e4](https://codeberg.org/gbrennon/senkai/commit/381a7e40c332390ed1d1f3a2e6ef5fcea71b4f12))
- Merge pull request 'feat/custom-worktree-dir' (#9) from feat/custom-worktree-dir into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/9
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([5c7e8e2](https://codeberg.org/gbrennon/senkai/commit/5c7e8e2d92f4b04dcc21d4beff865c64617b0a2f))
- Merge branch 'main' into feat/custom-worktree-dir ([734a0c5](https://codeberg.org/gbrennon/senkai/commit/734a0c53dcfb333a71b623d15309b8f9111da13f))
- Merge pull request 'docs/contributors.md' (#10) from feat/custom-worktree-dir into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/10
Reviewed-by: personal-reviewer <personal-reviewer@noreply.codeberg.org> ([9d202b4](https://codeberg.org/gbrennon/senkai/commit/9d202b4996e38c24a44f9c68fd98d038015939d4))

## [0.1] - 2026-05-23


### Added
- Impl auto-sync repo when worktree is deleted ([0fe13d0](https://codeberg.org/gbrennon/senkai/commit/0fe13d03a4a5013838e7141dfde993a5dc50dcc9))
- Feature commit ([fd507e2](https://codeberg.org/gbrennon/senkai/commit/fd507e2ab41d33317e0af36a49dfc4985c1ca06e))
- Srp applied to shell scripts ([7ba3d13](https://codeberg.org/gbrennon/senkai/commit/7ba3d13d73ee62751e116b40c591afb40f490db9))


### Changed
- Responsible only to use support classes ([617e2a8](https://codeberg.org/gbrennon/senkai/commit/617e2a8667ea8c44394d107efbac2dbf5779eda4))


### Documentation
- Write README.md ([1eac6c3](https://codeberg.org/gbrennon/senkai/commit/1eac6c319152f1638f438c91785eb63fdd2ed7e8))
- Add details to docs ([e5f5c62](https://codeberg.org/gbrennon/senkai/commit/e5f5c62f6e5d9dd4679bda9a6b0a46c56ad29eea))
- Explicit instructions to customise keys ([2e82a91](https://codeberg.org/gbrennon/senkai/commit/2e82a914bd4ab318e21490ce4bb3d54a876a7a0e))
- Introduce testing section ([1a12e6e](https://codeberg.org/gbrennon/senkai/commit/1a12e6eee336a4eab1abe612e1b462c98c5ec2e8))


### Fixed
- Worktree-manager now removes worktree consistently ([94528fc](https://codeberg.org/gbrennon/senkai/commit/94528fcd58c990742089a13967e5db0fc458a418))
- Remove legacy command ([532e7fb](https://codeberg.org/gbrennon/senkai/commit/532e7fb8095934fd2bbe53f2f9ad034292eb9607))
- Unbinding old keys ([dcb8b52](https://codeberg.org/gbrennon/senkai/commit/dcb8b52024a6ca95f31df986a05d1bf1fd581bd7))
- Remove worktree actions is now showing msg only in the bar ([614c6c3](https://codeberg.org/gbrennon/senkai/commit/614c6c3086ca834e7acfa90e1f69f525b93b75e1))


### Maintenance
- Remove feat.txt ([4b343ef](https://codeberg.org/gbrennon/senkai/commit/4b343ef8bacd1b0cf8fbc25b193831c5b7148713))
- Add .gitmodules to install bats as a submodule ([3806f14](https://codeberg.org/gbrennon/senkai/commit/3806f146c579bf73b363ff9f52a29a5e360b6f9a))
- Make command rleated to test are defined ([742feb4](https://codeberg.org/gbrennon/senkai/commit/742feb4e157c51578109d62b4388073a34819114))


### Other
- Safe expererimentation ([f1311a3](https://codeberg.org/gbrennon/senkai/commit/f1311a314f0f3e810e6b53453b2d95bc42bfaf10))
- Safe expererimentation ([8b31af7](https://codeberg.org/gbrennon/senkai/commit/8b31af74477590041d6779a1a9cccdf8afa1c8ee))
- Checkpoint ([43bfd0f](https://codeberg.org/gbrennon/senkai/commit/43bfd0f3eac77207d40def04771755051225e153))
- Merge pull request 'docs: add details to docs' (#1) from chore/improve-docs into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/1 ([7838571](https://codeberg.org/gbrennon/senkai/commit/78385719a7f0a856ed004e976915f8528cc92c81))
- Merge pull request 'docs(README): explicit instructions to customise keys' (#2) from chore/explicit-instructions-custom-key into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/2 ([e806d46](https://codeberg.org/gbrennon/senkai/commit/e806d461a9d1ca1fbd32105d79f59d6755403fc5))
- Merge pull request 'fix: unbinding old keys' (#3) from fix/worktree-customization into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/3 ([3327628](https://codeberg.org/gbrennon/senkai/commit/3327628ac54d057fa9dd83e3f6271dc3440013f1))
- Merge pull request 'feat/remove-worktree' (#4) from feat/remove-worktree into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/4 ([4ab9be7](https://codeberg.org/gbrennon/senkai/commit/4ab9be749e3837a3edb66b40dd7f58e6a72dbb99))
- Merge pull request 'perf(scripts): remove worktree' (#5) from chore/increase-performance into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/5 ([ce7e242](https://codeberg.org/gbrennon/senkai/commit/ce7e24262f04e08709743ea8497c96eed48738f5))
- Merge pull request 'fix: remove worktree actions is now showing msg only in the bar' (#6) from fix/remove-worktree-msg into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/6 ([f4ec4d4](https://codeberg.org/gbrennon/senkai/commit/f4ec4d409d7aacf62693b989765c03f987842670))
- Defined ci.yml workflow ([c047472](https://codeberg.org/gbrennon/senkai/commit/c047472ca2e77504466c05fcfdadbd013ff4c227))
- Using apt-get ([37290e7](https://codeberg.org/gbrennon/senkai/commit/37290e77c992cd6300733ea8aae251b39184128d))
- Defined ci.yml workflow ([38bf6ca](https://codeberg.org/gbrennon/senkai/commit/38bf6caa31bf66ccf8a9bd0518e37c1052b5b198))
- Merge pull request 'chore/migrate-to-bats' (#7) from chore/migrate-to-bats into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/7 ([2a11679](https://codeberg.org/gbrennon/senkai/commit/2a11679db8476f26fa7a91af48945c973a967863))
- Merge branch 'main' into chore/migrate-to-bats ([2c230bb](https://codeberg.org/gbrennon/senkai/commit/2c230bb897bb164adac8cd745bee19239ad533a4))
- Merge branch 'chore/migrate-to-bats' of codeberg.org:gbrennon/tmux-worktree into chore/migrate-to-bats ([8053c55](https://codeberg.org/gbrennon/senkai/commit/8053c556ef8e6df62f4da344edf4a5cb69d253d6))
- Implemented install-dependencies action ([9e1205f](https://codeberg.org/gbrennon/senkai/commit/9e1205f78b1394314fcef2e96335b138a990eedf))
- Fix in test.yml ([eda815e](https://codeberg.org/gbrennon/senkai/commit/eda815edb1b66e5075a92b5aff5f44a4e53fd715))
- Merge pull request 'ci(forgejo): defined ci.yml workflow' (#8) from chore/migrate-to-bats into main

Reviewed-on: https://codeberg.org/gbrennon/tmux-worktree/pulls/8 ([89571a6](https://codeberg.org/gbrennon/senkai/commit/89571a6908b1d4bc14a2bca972428f73da12f4cc))


### Performance
- Remove worktree ([dad5037](https://codeberg.org/gbrennon/senkai/commit/dad50374fc2eca731ea9a6b1ec15939b614785d5))


### Tests
- Write tests ([c9d46d2](https://codeberg.org/gbrennon/senkai/commit/c9d46d28a99229a366dfa0d4d06200eb104779c8))
- Migrating tests to bats ([fb2c11a](https://codeberg.org/gbrennon/senkai/commit/fb2c11a874daa53f98301a42e4dbd249c2464639))
- Bats tests ([d8577c9](https://codeberg.org/gbrennon/senkai/commit/d8577c90d9aec047f94f6b4a5de9021c0543f21f))

