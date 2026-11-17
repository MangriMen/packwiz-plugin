# Packwiz plugin for Aether

Imports and updates [packwiz](https://packwiz.infra.link) modpacks in the
[Aether](https://github.com/MangriMen/aether) launcher. The importer takes the URL or the file
of a `pack.toml`; the updater runs `packwiz-installer` (through its bootstrap jar, pinned by
sha256) in the instance, before every launch when the instance's settings ask for it.

## Plugin API

Built for plugin API `0.5` with `aether-core-plugin-pdk`, which re-exports the API as
`aether_core_plugin_pdk::api`. The API is described in the launcher's
[`packages/aether-core-plugin-api/README.md`](https://github.com/MangriMen/aether/blob/main/packages/aether-core-plugin-api/README.md).

`[package.metadata]` of `packwiz/Cargo.toml` holds what goes into the manifest:

* `api_version` → `api.version`, a SemVer requirement checked against the launcher's API version;
* `host_functions` → `api.hostFunctions`, every host function the wasm imports. The launcher
  links only these; `cargo xtask package` fails when the wasm imports one that is not listed and
  warns about one that is listed but not imported.

To build against a local checkout of the launcher, swap the `git = …` of
`aether-core-plugin-pdk` in `packwiz/Cargo.toml` for the `path` given in the comment above it.

## Building

Needs the `wasm32-wasip1` target (`rustup target add wasm32-wasip1`).

```sh
cargo xtask package   # build, check the host functions, write the manifest and zip it
cargo xtask bundle    # the same without the zip
cargo xtask build     # only the wasm
cargo xtask clean
```

The package is `target/wasm32-wasip1/release/package/packwiz.zip` (the wasm, `manifest.json`
and `capabilities.json`; the unzipped files are in `target/wasm32-wasip1/release/bundle/`).
