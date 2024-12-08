// This file is automatically generated by build.rs
// Do not edit this file manually!

use horizon_plugin_api::{Pluginstate, LoadedPlugin, Plugin};
use std::collections::HashMap;

pub use player_lib;
pub use player_lib::*;
pub use player_lib::Plugin as player_lib_plugin;


// Invoke the macro with all discovered plugins
pub fn load_plugins() -> HashMap<String, (Pluginstate, Plugin)> {
    let plugins = crate::load_plugins!(
        player_lib
    );
    plugins
}
