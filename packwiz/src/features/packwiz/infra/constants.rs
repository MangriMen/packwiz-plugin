pub const PACKWIZ_INSTALLER_FILE_NAME: &str = "packwiz-installer.jar";

/// A jar the plugin runs, pinned to a release and to its digest.
///
/// The host downloads it with `download_to_cache` and refuses anything whose sha256 differs, so
/// a changed or replaced release asset never gets to run. A new release is taken on purpose:
/// download it, check it, and update the URL and the digest together.
pub struct Redistributable {
    pub name: &'static str,
    pub url: &'static str,
    pub sha256: &'static str,
}

pub const PACKWIZ_INSTALLER: Redistributable = Redistributable {
    name: "packwiz-installer",
    url: "https://github.com/packwiz/packwiz-installer/releases/download/v0.5.14/packwiz-installer.jar",
    sha256: "c9f646908d340d84773948a9a7d98bc1dae250d35e1016dc6e2b8459760b5598",
};

pub const PACKWIZ_INSTALLER_BOOTSTRAP: Redistributable = Redistributable {
    name: "packwiz-installer-bootstrap",
    url: "https://github.com/packwiz/packwiz-installer-bootstrap/releases/download/v0.0.3/packwiz-installer-bootstrap.jar",
    sha256: "a8fbb24dc604278e97f4688e82d3d91a318b98efc08d5dbfcbcbcab6443d116c",
};
