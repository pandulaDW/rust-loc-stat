use super::config::{Config, Language};
use super::parser::Parser;
use std::path::{Path, PathBuf};
use std::{fs, fs::File, io};
#[derive(Debug)]
pub struct HandlerWithLanguage {
    handler: File,
    language: Language,
}

/// Collect the relevant file handlers and returns a vector of file handlers with the identified languages
pub fn process_files(conf: &Config) -> io::Result<()> {
    let mut handlers = Vec::new();
    collect_file_handlers(&conf.directory, &mut handlers, &conf.excluded_dirs)?;

    for file_item in handlers {
        let buf_reader = io::BufReader::new(file_item.handler);
        let mut parser = Parser::new(file_item.language);
        parser.parse(buf_reader)?;

        println!("{:?}", parser);
    }

    Ok(())
}

// Walks the directory recursively, open and collects the file handlers while excluding the specified directories.
// If there's an error opening a file handle, that file will be omitted silently.
fn collect_file_handlers(
    dir: &Path,
    file_paths: &mut Vec<HandlerWithLanguage>,
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
                if let Ok(f) = File::open(&path) {
                    if let Some(language) = get_language_by_extension(&path) {
                        file_paths.push(HandlerWithLanguage {
                            handler: f,
                            language,
                        });
                    }
                }
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

// get the language by looking at the file extension. If the extension is unknown, None will be returned
fn get_language_by_extension(path: &Path) -> Option<Language> {
    if let Some(extension) = path.extension() {
        let ext_str = extension.to_str().unwrap_or("");
        match ext_str {
            "js" => Some(Language::Javascript),
            "ts" => Some(Language::Javascript),
            "jsx" => Some(Language::Typescript),
            "tsx" => Some(Language::Typescript),
            _ => None,
        }
    } else {
        None
    }
}
