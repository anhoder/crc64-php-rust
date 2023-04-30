use crate::{
    dual_crc::{make_dual_crc_class,},
};
use phper::{modules::Module, php_get_module};

pub mod dual_crc;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_class(make_dual_crc_class());

    module
}
