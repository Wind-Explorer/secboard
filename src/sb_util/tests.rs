#[test]
fn test_ensure_path_exists() -> Result<(), Box<dyn std::error::Error>> {
    use std::{env::temp_dir, fs};
    let temp_dir = temp_dir();
    let test_dir = temp_dir.join("test_dir_uwu");
    let test_file = test_dir.join("test_file.txt");

    // Ensure the directory and file don't exist
    if test_file.exists() {
        fs::remove_file(&test_file)?;
    }
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir)?;
    }

    // Ensure the directory and file are created
    let result_dir = crate::sb_util::paths::ensure_path_exists(&test_dir, false)?;
    let result_file = crate::sb_util::paths::ensure_path_exists(&test_file, true)?;

    // Check that the returned paths match the expected paths
    assert_eq!(result_dir, test_dir);
    assert_eq!(result_file, test_file);

    // Ensure the directory and file still exist
    assert!(test_dir.exists());
    assert!(test_file.exists());

    // Clean up the test directory and file
    fs::remove_file(&test_file)?;
    fs::remove_dir_all(&test_dir)?;

    Ok(())
}
