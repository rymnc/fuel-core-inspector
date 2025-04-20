//! simple hex utils

use fuel_core_storage::kv_store::Value;
use pretty_hex::{
    HexConfig,
    config_hex,
};

/// Hex printing extension trait
pub trait PrintHexExt {
    /// Print the value in hexadecimal format
    fn print(&self);
}

impl PrintHexExt for Option<Value> {
    fn print(&self) {
        match self {
            Some(value) => {
                let cfg = HexConfig {
                    width: 20,
                    group: 2,
                    ..HexConfig::default()
                };
                println!("{}", config_hex(&value, cfg));
            }
            None => println!("No value found"),
        }
    }
}
