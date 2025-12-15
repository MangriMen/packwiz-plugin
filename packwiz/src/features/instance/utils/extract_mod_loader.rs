use aether_core_plugin_api::v0::ModLoaderDto;

use crate::features::packwiz::PackVersions;

pub fn extract_mod_loader(version: &PackVersions) -> crate::Result<(ModLoaderDto, Option<String>)> {
    match (
        version.fabric.as_ref(),
        version.forge.as_ref(),
        version.liteloader.as_ref(),
        version.quilt.as_ref(),
    ) {
        (Some(fabric), _, _, _) => Ok((ModLoaderDto::Fabric, Some(fabric.clone()))),
        (_, Some(forge), _, _) => Ok((ModLoaderDto::Forge, Some(forge.clone()))),
        (_, _, Some(liteloader), _) => Err(crate::Error(format!(
            "Unsupported mod loader: \"liteloader: {}\"",
            liteloader
        ))),
        (_, _, _, Some(quilt)) => Ok((ModLoaderDto::Quilt, Some(quilt.clone()))),
        (_, _, _, _) => Ok((ModLoaderDto::Vanilla, None)),
    }
}
