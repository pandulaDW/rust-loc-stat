use std::path::PathBuf;

/// Language includes all the languages supported by the application
#[derive(Debug)]
pub enum Language {
    Javascript,
    Typescript,
}

/// Contains the configuration data of the application.
///
/// ### Fields:
/// - directory is the user provided code directory path.
/// - excluded_dirs includes directories that would be excluded by the application.
pub struct Config {
    pub directory: PathBuf,
    pub excluded_dirs: Vec<PathBuf>,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let mut p = PathBuf::new();
        p.push(path);

        // TODO - get this by configuration file
        let excluded_dirs = create_excluded_paths(path, vec![".git", "target", "node_modules"]);
        println!("excluded -> {:?}", excluded_dirs);

        Config {
            directory: p,
            excluded_dirs,
        }
    }
}

// create a vector of paths based on the excluded directories by appending the root path provided
fn create_excluded_paths(root: &str, dirs: Vec<&str>) -> Vec<PathBuf> {
    dirs.iter()
        .map(|v| {
            let mut p = PathBuf::new();
            p.push(root);
            p.push(v);
            p
        })
        .collect()
}
