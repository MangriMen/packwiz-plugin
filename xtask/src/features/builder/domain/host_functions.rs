/// How the `api.hostFunctions` a plugin declares lines up with what its wasm imports.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct HostFunctionCheck {
    /// Imported but not declared: the launcher will refuse to load the plugin.
    pub undeclared: Vec<String>,
    /// Declared but never imported: granted to the plugin for nothing.
    pub unused: Vec<String>,
}

impl HostFunctionCheck {
    pub fn compare(imported: &[String], declared: &[String]) -> Self {
        let missing_from = |names: &[String], other: &[String]| {
            let mut missing: Vec<String> = names
                .iter()
                .filter(|name| !other.contains(name))
                .cloned()
                .collect();
            missing.sort();
            missing.dedup();
            missing
        };

        Self {
            undeclared: missing_from(imported, declared),
            unused: missing_from(declared, imported),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(names: &[&str]) -> Vec<String> {
        names.iter().map(|&name| name.to_owned()).collect()
    }

    #[test]
    fn should_pass_when_the_declaration_matches_the_imports() {
        let check =
            HostFunctionCheck::compare(&names(&["log", "get_id"]), &names(&["get_id", "log"]));

        assert_eq!(check, HostFunctionCheck::default());
    }

    #[test]
    fn should_report_imports_missing_from_the_declaration() {
        let check = HostFunctionCheck::compare(&names(&["log", "run_command"]), &names(&["log"]));

        assert_eq!(check.undeclared, ["run_command"]);
        assert!(check.unused.is_empty());
    }

    #[test]
    fn should_report_declarations_nothing_imports() {
        let check = HostFunctionCheck::compare(
            &names(&["log"]),
            &names(&["log", "http_post_json", "http_post_json"]),
        );

        assert!(check.undeclared.is_empty());
        assert_eq!(check.unused, ["http_post_json"]);
    }
}
