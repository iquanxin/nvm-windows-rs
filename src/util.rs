use std::path::PathBuf;

pub fn is_version_installed(path: &PathBuf) -> bool {
    if path.exists() {
        return true;
    }
    return false;
}
