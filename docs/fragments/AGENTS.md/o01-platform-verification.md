## Platform verification

- The `sysinfo` crate lives at the repository root. Its platform implementations live under `src/unix/`, `src/windows/` and `src/unknown/`. Preserve public feature combinations, including builds without default features and the unknown-platform behavior.
- Use Rust `1.100` or newer for the workspace and the standalone `test-unknown` fixture. The `basic` benchmark uses nightly `libtest`. Build the fixture with `wasm-pack build --target web` from `test-unknown/`, and build the C example with `make` from the root.
- The workflows in `.github/workflows/` define the platform and feature test matrix. Cross-compilation checks the selected target's code and dependency graph. Platform runtime tests exercise the native behavior. The upstream CI profile already accounts for runners that lack thermal sensors.
