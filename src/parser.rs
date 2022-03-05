use super::config;
use std::io::{BufRead, BufReader, Read};

/// Parser encapsulates information about the currently parsed file
///
/// line_stats tuple corresponds to (code_lines, comment_lines, empty_lines)
pub struct Parser {
    line_stats: (u32, u32, u32),
    is_within_multiline_comment: bool,
    language: config::Language,
}

impl Parser {
    /// Constructs a new parser.
    pub fn new(language: config::Language) -> Self {
        Parser {
            line_stats: (0, 0, 0),
            is_within_multiline_comment: false,
            language,
        }
    }

    /// Parses the input from the given buffered reader
    ///
    /// The generic parameter R corresponds to the type of the reader that the application uses. (eg. file, bytes etc)
    pub fn parse<R: Read>(&mut self, reader: BufReader<R>) {
        for read in reader.lines() {
            if let Ok(line) = read {
                self.parse_line(&line);
            } else {
            }
        }
    }

    /// Parses the given line and updates the loc_stats
    fn parse_line(&mut self, line: &str) {
        let trimmed = line.trim();
        if self.parse_comments(trimmed) {
            self.line_stats.2 += 1;
        } else if trimmed.is_empty() {
            self.line_stats.1 += 1;
        } else {
            self.line_stats.0 += 1;
        }
    }

    /// Checks whether the line is a comment. The input will be a trimmed line.
    ///
    /// Set or unset a multiline comment block as well.
    fn parse_comments(&mut self, line: &str) -> bool {
        use config::Language::*;
        match self.language {
            Javascript => {
                if self.is_within_multiline_comment {
                    if line.contains("*/") {
                        self.is_within_multiline_comment = false;
                    }
                    return true;
                } else {
                    if line.starts_with("/*") {
                        self.is_within_multiline_comment = true;
                        return true;
                    } else if line.starts_with("//") {
                        return true;
                    }
                    return false;
                }
            }
        }
    }
}
