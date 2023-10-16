use std::fs::{self, File};
use std::path::{Path, PathBuf};

///
/// Ensure that directory or file passed in exists
/// If it doesn't, create it
/// If it does, do nothing
///
/// ## Arguments
/// * `path` - Path to directory or file to ensure exists
/// * `is_file` - Optional flag to indicate if the path is a file (default is false)
///
/// ## Returns
/// * `Result<PathBuf, Box<dyn std::error::Error>>` - Value of path passed in
///
pub fn ensure_path_exists(
    path: &Path,
    is_file: bool,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if !path.exists() {
        if is_file {
            let parent = path.parent().unwrap_or_else(|| Path::new(""));
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
            File::create(path)?;
        } else {
            fs::create_dir_all(path)?;
        }
    }
    Ok(path.to_path_buf())
}
