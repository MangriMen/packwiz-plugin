use std::path::Path;

use aether_core_plugin_api::v0::CommandDto;

use crate::features::{
    host::{self, log, LogLevel},
    instance::{
        disable_contents, enable_contents, instance_plugin_get_dir, list_content, CommandDtoExt,
        PackwizSettings,
    },
    packwiz,
};

pub fn update(instance_id: String) -> crate::Result<()> {
    packwiz::preload_resources(&instance_id)?;

    let instance_plugin_folder = instance_plugin_get_dir(instance_id.clone())
        .map_err(|_| crate::Error("Failed to get instance directory".to_owned()))?;

    let packwiz_path = Path::new(&instance_plugin_folder).join("packwiz.toml");
    let instance_packwiz_settings = crate::api::settings::get_from_path(&packwiz_path)?;

    update_pack_base(&instance_id, &instance_packwiz_settings)?;

    Ok(())
}

pub fn update_pack_base(instance_id: &str, settings: &PackwizSettings) -> crate::Result<()> {
    log(
        LogLevel::Info,
        format!("Updating pack in instance {}", &instance_id),
    );

    let command = packwiz::get_command_to_update_pack(instance_id, settings)
        .map(|command| CommandDto::from_command(&command))?;

    let disabled_contents_paths = get_disabled_contents_paths(instance_id)?;
    let should_temporary_enable_contents = !disabled_contents_paths.is_empty();

    if should_temporary_enable_contents {
        enable_contents(instance_id.to_owned(), disabled_contents_paths.clone())?;
    }

    host::run_command(command.clone())?;

    if should_temporary_enable_contents {
        disable_contents(instance_id.to_owned(), disabled_contents_paths)?
    }

    log(
        LogLevel::Info,
        format!("Pack in instance {} updated successfully!", &instance_id),
    );

    Ok(())
}

fn get_disabled_contents_paths(instance_id: &str) -> crate::Result<Vec<String>> {
    let contents = list_content(instance_id.to_owned())?;

    let contents_values_iter = contents.iter().map(|content| content.1);

    let disabled_contents = contents_values_iter
        .filter_map(|content| {
            if content.disabled {
                Some(content.content_path.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(disabled_contents)
}
