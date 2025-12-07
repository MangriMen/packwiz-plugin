use crate::features::{instance::ModLoader, packwiz::PackVersions};

pub fn extract_mod_loader(version: &PackVersions) -> crate::Result<(ModLoader, Option<String>)> {
    match (
        version.fabric.as_ref(),
        version.forge.as_ref(),
        version.liteloader.as_ref(),
        version.quilt.as_ref(),
    ) {
        (Some(fabric), _, _, _) => Ok((ModLoader::Fabric, Some(fabric.clone()))),
        (_, Some(forge), _, _) => Ok((ModLoader::Forge, Some(forge.clone()))),
        (_, _, Some(liteloader), _) => Err(crate::Error(format!(
            "Unsupported mod loader: \"liteloader: {}\"",
            liteloader
        ))),
        (_, _, _, Some(quilt)) => Ok((ModLoader::Quilt, Some(quilt.clone()))),
        (_, _, _, _) => Ok((ModLoader::Vanilla, None)),
    }
}
