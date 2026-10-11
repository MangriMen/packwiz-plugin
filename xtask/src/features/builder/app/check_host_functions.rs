use std::path::Path;

use wasmparser::{Parser, Payload, TypeRef};

use crate::features::builder::HostFunctionCheck;

/// Import module Extism puts the core's host functions in. `extism:host/env` (the Extism
/// kernel) and `wasi_snapshot_preview1` are not host functions of the core and are never
/// declared.
const HOST_FUNCTION_MODULE: &str = "extism:host/user";

/// Checks the built wasm against `api.hostFunctions` (aether T-1.4.B, F-46).
///
/// The launcher refuses to load a plugin that imports a host function it did not declare, so
/// that is a build error here. A declared function the wasm never imports is only a warning:
/// it loads fine, but grants the plugin something it does not use.
pub fn check_host_functions(wasm_path: &Path, declared: &[String]) -> Result<(), String> {
    let imported = read_host_function_imports(wasm_path)?;
    let check = HostFunctionCheck::compare(&imported, declared);

    if !check.unused.is_empty() {
        eprintln!(
            "warning: `host_functions` in [package.metadata] declares functions the wasm never \
             imports: {}. Drop them so the plugin is not granted what it does not use.",
            check.unused.join(", ")
        );
    }

    if !check.undeclared.is_empty() {
        return Err(format!(
            "the wasm imports host functions missing from `host_functions` in \
             [package.metadata]: {}. The launcher refuses to load a plugin that imports an \
             undeclared host function.",
            check.undeclared.join(", ")
        ));
    }

    Ok(())
}

fn read_host_function_imports(wasm_path: &Path) -> Result<Vec<String>, String> {
    let bytes = std::fs::read(wasm_path)
        .map_err(|err| format!("can't read {}: {err}", wasm_path.display()))?;
    let invalid =
        |err: wasmparser::BinaryReaderError| format!("can't parse {}: {err}", wasm_path.display());

    let mut imported = Vec::new();

    for payload in Parser::new(0).parse_all(&bytes) {
        let Payload::ImportSection(section) = payload.map_err(invalid)? else {
            continue;
        };

        for import in section.into_imports() {
            let import = import.map_err(invalid)?;
            if import.module == HOST_FUNCTION_MODULE && matches!(import.ty, TypeRef::Func(_)) {
                imported.push(import.name.to_owned());
            }
        }
    }

    Ok(imported)
}
