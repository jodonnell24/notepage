use std::path::Path;

// Ensure a directory exists, creating it and all parent directories if needed
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    // create_dir_all creates the directory and any missing parent directories
    std::fs::create_dir_all(path)?;

    Ok(())
}

// Copy a file from one location to another
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> anyhow::Result<u64> {
    std::fs::copy(from, to).map_err(Into::into)
}