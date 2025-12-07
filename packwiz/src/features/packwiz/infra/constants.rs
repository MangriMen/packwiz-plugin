use phf::phf_map;

pub const PACKWIZ_INSTALLER_FILE_NAME: &str = "packwiz-installer.jar";
pub const PACKWIZ_INSTALLER_BOOTSTRAP_FILE_NAME: &str = "packwiz-installer-bootstrap.jar";

pub const REDISTRIBUTABLE_FILES: phf::Map<&'static str, &str> = phf_map! {
  "packwiz-installer" => PACKWIZ_INSTALLER_FILE_NAME,
  "packwiz-installer-bootstrap" => PACKWIZ_INSTALLER_BOOTSTRAP_FILE_NAME
};

pub const REDISTRIBUTABLE_LINKS: phf::Map<&'static str, &str> = phf_map! {
  "packwiz-installer" => "https://github.com/packwiz/packwiz-installer/releases/latest/download/packwiz-installer.jar",
  "packwiz-installer-bootstrap" => "https://github.com/packwiz/packwiz-installer-bootstrap/releases/latest/download/packwiz-installer-bootstrap.jar"
};
