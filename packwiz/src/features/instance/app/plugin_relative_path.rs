use crate::features::instance::{instance_get_dir, instance_plugin_get_dir};

/// Path of `file_name` inside this plugin's per-instance directory, **relative to the instance
/// root** — the form `write_instance_file` takes (T-1.5.A.1).
///
/// The plugin only knows its own directory as a plugin-space path
/// (`/instances/<id>/.metadata/plugins/<plugin_id>`), which the host function does not accept.
/// Both that path and the instance root come from the same host and the same space, so the
/// relative form is the second with the first stripped off. Computed rather than hardcoded:
/// neither this plugin's id nor the core's layout of `.metadata` belongs in the plugin.
pub fn instance_plugin_relative_path(instance_id: &str, file_name: &str) -> crate::Result<String> {
    let instance_dir = instance_get_dir(instance_id)?;
    let plugin_dir = instance_plugin_get_dir(instance_id)?;

    // `starts_with('/')` and not just `strip_prefix`: without it an instance `a` would look like
    // a prefix of an instance `ab`, and the relative path would silently point elsewhere.
    let relative = plugin_dir
        .strip_prefix(instance_dir.as_str())
        .filter(|rest| rest.starts_with('/'))
        .map(|rest| rest.trim_start_matches('/'))
        .filter(|rest| !rest.is_empty())
        .ok_or_else(|| {
            format!("Plugin directory {plugin_dir} is not inside instance directory {instance_dir}")
        })?;

    Ok(format!("{relative}/{file_name}"))
}
