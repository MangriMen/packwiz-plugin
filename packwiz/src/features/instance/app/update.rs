use std::path::Path;

use aether_core_plugin_pdk::api::v0::{CommandDto, OutputDto};

use crate::{
    ResultExt,
    features::{
        host::{self, LogLevel, log},
        instance::infra::CommandDtoExt,
        instance::{
            PackwizSettings, disable_contents, enable_contents, instance_plugin_get_dir,
            list_content,
        },
        packwiz,
    },
};

pub fn update(instance_id: String) -> crate::Result<()> {
    let redistributables = packwiz::preload_resources(&instance_id)?;

    let instance_plugin_folder = instance_plugin_get_dir(&instance_id)
        .inspect_err(|e| log(LogLevel::Error, format!("{e}")))
        .context("Failed to get instance directory")?;

    let packwiz_path = Path::new(&instance_plugin_folder).join("packwiz.toml");
    let instance_packwiz_settings = crate::api::settings::get_from_path(&packwiz_path)?;

    update_pack_base(
        &instance_id,
        &instance_packwiz_settings,
        &redistributables.bootstrap,
    )?;

    Ok(())
}

pub fn update_pack_base(
    instance_id: &str,
    settings: &PackwizSettings,
    bootstrap: &Path,
) -> crate::Result<()> {
    log(
        LogLevel::Info,
        format!("Updating pack in instance {instance_id}"),
    );

    let command = packwiz::get_command_to_update_pack(instance_id, settings, bootstrap)
        .map(|command| CommandDto::from_command(&command))?;

    let disabled_contents_paths = get_disabled_contents_paths(instance_id)?;
    let should_temporary_enable_contents = !disabled_contents_paths.is_empty();

    if should_temporary_enable_contents {
        enable_contents(instance_id, disabled_contents_paths.clone())
            .inspect_err(|e| log(LogLevel::Error, format!("{e}")))
            .context("Failed to enable contents")?;
    }

    // Whatever the installer did, the contents the user had disabled are disabled again before
    // its result is looked at.
    let updated = host::run_command(command).and_then(|output| installer_succeeded(&output));

    if should_temporary_enable_contents {
        disable_contents(instance_id, disabled_contents_paths)
            .inspect_err(|e| log(LogLevel::Error, format!("{e}")))
            .context("Failed to disable contents")?;
    }

    updated?;

    log(
        LogLevel::Info,
        format!("Pack in instance {instance_id} updated successfully!"),
    );

    Ok(())
}

/// `run_command` answers a non-zero exit like any other: the installer failed when it did not
/// exit with 0, and what it printed says why.
fn installer_succeeded(output: &OutputDto) -> crate::Result<()> {
    if output.success() {
        return Ok(());
    }

    let exit = match output.exit_code {
        Some(code) => format!("exited with code {code}"),
        None => "was killed".to_owned(),
    };

    // packwiz-installer reports some failures on stdout; stderr is preferred when it has any.
    let (stream, truncated) = if output.stderr.iter().any(|b| !b.is_ascii_whitespace()) {
        (&output.stderr, output.stderr_truncated)
    } else {
        (&output.stdout, output.stdout_truncated)
    };
    let text = String::from_utf8_lossy(stream);
    let text = text.trim();

    let message = match (text.is_empty(), truncated) {
        (true, _) => format!("packwiz-installer {exit}"),
        (false, false) => format!("packwiz-installer {exit}: {text}"),
        (false, true) => format!("packwiz-installer {exit}: {text} (output cut short)"),
    };

    log(LogLevel::Error, message.clone());
    Err(message.into())
}

fn get_disabled_contents_paths(instance_id: &str) -> crate::Result<Vec<String>> {
    let contents = list_content(instance_id)
        .inspect_err(|e| log(LogLevel::Error, format!("{e}")))
        .context("Failed to list contents")?;

    let disabled_contents = contents
        .values()
        .filter(|content| content.disabled)
        .map(|content| content.content_path.to_owned())
        .collect();

    Ok(disabled_contents)
}
