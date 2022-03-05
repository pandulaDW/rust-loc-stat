use super::config;
use std::io::{self, BufRead, BufReader, Read};

/// Parser encapsulates information about the currently parsed file
///
/// line_stats tuple corresponds to (code_lines, empty_lines, comment_lines)
#[derive(Debug)]
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

    /// Parses the input from the given buffered reader and returns an error if found.
    ///
    /// The generic parameter R corresponds to the type of the reader that the application uses. (eg. file, bytes etc)
    pub fn parse<R: Read>(&mut self, reader: BufReader<R>) -> io::Result<()> {
        for read in reader.lines() {
            let line = read?;
            self.parse_line(&line);
        }
        Ok(())
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
    /// It sets or unset a multiline comment block as well.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comments() {
        let mut js_parser = Parser::new(config::Language::Javascript);
        assert!(js_parser.parse_comments("/*"));
        assert!(js_parser.is_within_multiline_comment);

        assert!(js_parser.parse_comments("*/ // another comment"));
        assert!(!js_parser.is_within_multiline_comment);

        assert!(js_parser.parse_comments("// some comment"));
    }

    #[test]
    fn test_parse_input_block() {
        let mut parser = Parser::new(config::Language::Javascript);
        let code_block = r"/*
            This is multi comment block
        */
    
        const add = (a, b) => {
        return a + b;
        };
        
        // another comment
        
        add(2, 3);";

        let input = Vec::from(code_block);
        let reader = BufReader::new(input.as_slice());
        let result = parser.parse(reader);
        assert!(result.is_ok());

        assert_eq!(parser.line_stats, (4, 3, 4));
        assert!(!parser.is_within_multiline_comment);
    }
}
