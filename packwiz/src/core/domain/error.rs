//! The plugin's error is the SDK's: a host error keeps its kind on the way back to the launcher
//! (a refused command or host reaches the user as such), and `ResultExt::context` puts the
//! plugin's words in front of it.
pub use aether_core_plugin_pdk::{Error, Result, ResultExt};
