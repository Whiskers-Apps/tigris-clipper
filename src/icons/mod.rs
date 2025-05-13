use std::path::PathBuf;

use tigris_core::features::extensions::get_extension_dir;

use crate::EXTENSION_ID;

pub fn get_icon_path(name: &str) -> PathBuf {
    PathBuf::from(get_extension_dir(EXTENSION_ID).unwrap())
        .join("src")
        .join("icons")
        .join(format!("{name}.svg"))
}
