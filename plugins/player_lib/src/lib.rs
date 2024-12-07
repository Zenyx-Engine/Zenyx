use std::collections::HashMap;

pub use horizon_plugin_api::{LoadedPlugin, Plugin, Pluginstate};

// Define the trait properly
pub trait PluginAPI {
    fn test(&self);
}

pub trait PluginConstruct {
    fn get_structs(&self) -> Vec<&str>;
    // If you want default implementations, mark them with 'default'
    fn new(plugins: HashMap<String, (Pluginstate, Plugin)>) -> Plugin;
}

// Implement constructor separately
impl PluginConstruct for Plugin {
    fn new(plugins: HashMap<String, (Pluginstate, Plugin)>) -> Plugin {
        Plugin {}
    }

    fn get_structs(&self) -> Vec<&str> {
        vec!["MyPlayer"]
    }
}

// Implement the trait for Plugin
impl PluginAPI for Plugin {
    fn test(&self) {
        println!("test");
    }
}

//-----------------------------------------------------------------------------
// Plugin Implementation
//-----------------------------------------------------------------------------
