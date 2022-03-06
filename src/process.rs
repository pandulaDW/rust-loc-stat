use super::config::Language;
use super::parser::{LineStats, Parser};
use std::path::{Path, PathBuf};
use std::{fs, fs::File, io};

/// Responsible in processing all the files, aggregating the results and displaying
pub struct Processor {
    pub aggregated_result: LineStats,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            aggregated_result: (0, 0, 0),
        }
    }

    /// Walks the directory recursively, open each file handler found and process it in a sequence.
    ///
    /// Handlers will be dropped one after the other, to avoid panicking after having too many open
    /// file handlers.
    ///
    /// If there's an error in handling a file, that file will be omitted silently.
    pub fn process_files(&mut self, dir: &Path, excluded_dirs: &Vec<PathBuf>) -> io::Result<()> {
        if dir.is_dir() {
            if is_dir_excluded(dir, excluded_dirs) {
                return Ok(());
            }

            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    self.process_files(&path, excluded_dirs)?;
                } else {
                    if let Ok(f) = File::open(&path) {
                        if let Some(language) = get_language_by_extension(&path) {
                            self.process_file(f, language)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // process the individual file. Parses and then aggregate the result
    fn process_file(&mut self, handler: File, language: Language) -> io::Result<()> {
        let buf_reader = io::BufReader::new(handler);
        let mut parser = Parser::new(language);
        parser.parse(buf_reader)?;
        self.aggregate_result(parser);
        Ok(())
    }

    fn aggregate_result(&mut self, parser: Parser) {
        self.aggregated_result.0 += parser.line_stats.0;
        self.aggregated_result.1 += parser.line_stats.1;
        self.aggregated_result.2 += parser.line_stats.2;
    }
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
            "js" | "jsx" => Some(Language::Javascript),
            "ts" | "tsx" => Some(Language::Typescript),
            _ => None,
        }
    } else {
        None
    }
}
