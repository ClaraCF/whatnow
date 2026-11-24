use std::{io, fs};
use std::env;
use std::path::{Path, PathBuf};

pub fn can_read_dir(dir: &Path) -> io::Result<bool> {
    match fs::read_dir(dir) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => Ok(false),
        Err(e) => Err(e),
    }
}

#[cfg(windows)]
fn get_config_home() -> Option<PathBuf> {

}

#[cfg(unix)]
pub fn get_config_home() -> Option<PathBuf> {
    // Check if $XDG_CONFIG_HOME is set
    if let Some(xdg_config_home) = env::var_os("XDG_CONFIG_HOME") {
        // Check if $XDG_CONFIG_HOME is set to a valid dir we can read
        let path = Path::new(&xdg_config_home);
        if path.is_dir() && can_read_dir(path).is_ok() {
            // Check if whatnow exists in the $XDG_CONFIG_HOME path
            let path = PathBuf::from(path).join("whatnow");
            if path.is_dir() && can_read_dir(path.as_path()).is_ok() {
                return Some(path);
            }

            // $XDG_CONFIG_HOME exists, but doesn't contain whatnow. Try to create it
            if fs::create_dir_all(&path).is_ok() {
                return Some(path);
            }
        }
    }

    // Check $HOME/.config/
    if let Some(home) = env::var_os("HOME") {
        // Check if $HOME/.config/ is a valid dir we can read
        let path = PathBuf::from(home).join(".config");
        if path.is_dir() && can_read_dir(path.as_path()).is_ok() {
            // Check if whatnow exists in $HOME/.config
            let path = path.join("whatnow");
            if path.is_dir() && can_read_dir(path.as_path()).is_ok() {
                return Some(path);
            }

            // $HOME/.config exists, but doesn't contain whatnow. Try to create it
            if fs::create_dir_all(&path).is_ok() {
                return Some(path);
            }
        }
    }

    // No config dir available
    None
}

#[cfg(unix)]
pub fn get_data_home() -> Option<PathBuf> {
    // Check if $XDG_DATA_HOME is set
    if let Some(xdg_config_home) = env::var_os("XDG_DATA_HOME") {
        // Check if $XDG_DATA_HOME is set to a valid dir we can read
        let path = Path::new(&xdg_config_home);
        if path.is_dir() && can_read_dir(path).is_ok() {
            // Check if whatnow exists in the $XDG_DATA_HOME path
            let path = PathBuf::from(path).join("whatnow");
            if path.is_dir() && can_read_dir(path.as_path()).is_ok() {
                return Some(path);
            }

            // $XDG_DATA_HOME exists, but doesn't contain whatnow. Try to create it
            if fs::create_dir_all(&path).is_ok() {
                return Some(path);
            }
        }
    }

    // Check $HOME/.local/share
    if let Some(home) = env::var_os("HOME") {
        // Check if $HOME/.local/share/ is a valid dir we can read
        let path = PathBuf::from(home).join(".local").join("share");
        if path.is_dir() && can_read_dir(path.as_path()).is_ok() {
            // Check if whatnow exists in $HOME/.local/share
            let path = path.join("whatnow");
            if path.is_dir() && can_read_dir(path.as_path()).is_ok() {
                return Some(path);
            }

            // $HOME/.local/share exists, but doesn't contain whatnow. Try to create it
            if fs::create_dir_all(&path).is_ok() {
                return Some(path);
            }
        }
    }

    // No data dir available
    None
}
