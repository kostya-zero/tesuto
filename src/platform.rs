use std::env;
use std::path::{Path, PathBuf};
use crate::platform::PlatformName::Windows;

#[derive(PartialEq)]
pub enum PlatformName {
    Windows,
    Linux,
    Mac,
    Unknown,
}

pub struct Platform;
impl Platform {
    pub fn get_platform() -> PlatformName {
        match env::consts::OS {
            "windows" => PlatformName::Windows,
            "linux" => PlatformName::Linux,
            "macos" => PlatformName::Mac,
            _ => PlatformName::Unknown,
        }
    }

    pub fn get_paths() -> Vec<String> {
        let paths = env::var("PATH").unwrap();
        if Self::get_platform() == Windows {
            paths.split(';').map(|i| i.to_string()).collect()
        } else {
            paths.split(':').map(|i| i.to_string()).collect()
        }
    }

    pub fn is_program_installed(program: &str) -> bool {
        let paths_vec = Self::get_paths();
        let str_path: PathBuf = if Self::get_platform() == Windows {
            Path::new(&program).with_extension("exe")
        } else {
            Path::new(&program).to_path_buf()
        };

        if str_path.is_absolute() {
            return str_path.exists();
        }

        for path in paths_vec {
            let temp_path = Path::new(&path).join(str_path.clone());
            if temp_path.exists() {
                return true;
            }
        }
        false
    }
}