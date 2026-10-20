# Pull request notes

- Adds Wild as a submodule and builds its binary through the corresponding `rust.wild` bootstrap option.
- Enables Wild with `-Clinker-features=+wild`, which requires `-Zunstable-options`.
- Tested locally with Crater on approximately 6000 crates, with only one minor issue: https://github.com/wild-linker/wild/issues/2756.
- Available only for the `x86_64`, `aarch64`, `riscv64gc`, and `loongarch64` GNU ELF targets.
- Intentionally supports only the bundled Wild linker (`-Clink-self-contained=+linker`). Wild is still under development and is not yet widely available in stable distributions.
  Consequently, `-Clink-self-contained=-linker` is ignored when `-Clinker-features=+wild` is enabled.
- Uses an approach similar to the LLD integration: `ld.bfd` points to the Wild binary, and the compiler driver invokes it with `-fuse-ld=bfd`.

## Questions for the Rust community

- Would it be possible to run a full Crater test, ideally across all supported platforms, once this lands on `main`?
- Could we also run rustc-perf benchmarks?
- Is it acceptable to include Wild as a submodule pinned to a stable release tag?
- The newly introduced `-Clinker-flavor` values are not documented. Is that acceptable? This option appears to be obsolete, and some existing values are also undocumented.

## Questions (to Wild developers)

- The submodule integration is elegant, but it also pulls in mold and other submodules. Can we avoid including those dependencies?
- Using a single codegen unit in `profile.release` significantly slows down bootstrapping. Could we adjust this for the Rust integration?
  (Or should we add a new binary crate to the project depending on `libwild` crate)?

## TODOs

- `#rust.bootstrap-override-wild = true` - can be added once beta compiler supports `-Clinker-features=+wild`

## Known LLD compatibility difference

- Wild defaults to `-z now` (eager symbol binding) and does not support `-z lazy`.
- Identical code folding (ICF) is not yet implemented: https://github.com/wild-linker/wild/issues/1595.
- Linker script compatibility is incomplete. See https://github.com/wild-linker/wild/blob/main/LINKER_SCRIPT_SUPPORT.md.
- The `--emit-relocs` option, useful for binary optimization tools such as BOLT, is not yet supported.
- Some layout and profile-guided optimization options, such as `--call-graph-ordering-file` and `--shuffle-sections`, are not yet supported. These are additional options that `ld.bfd` does not support either.
