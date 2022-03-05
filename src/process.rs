use super::config::Config;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Collect the relevant file handlers and returns a Result
pub fn process_files(conf: &Config) -> io::Result<Vec<io::Result<File>>> {
    let mut handlers = Vec::new();
    collect_file_handlers(&conf.directory, &mut handlers, &conf.excluded_dirs)?;
    Ok(handlers)
}

// Walks the directory recursively, open and collects the file handlers while excluding the relevant directories
fn collect_file_handlers(
    dir: &Path,
    file_paths: &mut Vec<io::Result<File>>,
    excluded_dirs: &Vec<PathBuf>,
) -> io::Result<()> {
    if dir.is_dir() {
        if is_dir_excluded(dir, excluded_dirs) {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                collect_file_handlers(&path, file_paths, excluded_dirs)?;
            } else {
                let f = fs::File::open(&path);
                println!("{:?}", path);
                file_paths.push(f);
            }
        }
    }

    Ok(())
}

// check if given directory is an excluded directory
fn is_dir_excluded(dir: &Path, excluded_dirs: &Vec<PathBuf>) -> bool {
    for path in excluded_dirs.iter() {
        if dir.starts_with(path) {
            return true;
        }
    }
    false
}
