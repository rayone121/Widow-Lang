use crate::lexer::token::Token;
use logos::{Lexer as LogosLexer, Logos};
use std::ops::Range;

/// Position information for a token in the source code
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }
}

/// A token with its position information
#[derive(Debug, Clone, PartialEq)]
pub struct LocatedToken {
    pub token: Token,
    pub span: Range<usize>,
    pub start_pos: Position,
    pub end_pos: Position,
}

impl LocatedToken {
    pub fn new(token: Token, span: Range<usize>, start_pos: Position, end_pos: Position) -> Self {
        Self {
            token,
            span,
            start_pos,
            end_pos,
        }
    }
}

/// Enhanced lexer with position tracking and utility functions
pub struct WidowLexer<'a> {
    lexer: LogosLexer<'a, Token>,
    source: &'a str,
    current_line: usize,
    current_column: usize,
    last_newline_pos: usize,
}

impl<'a> WidowLexer<'a> {
    /// Create a new lexer for the given source code
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Token::lexer(source),
            source,
            current_line: 1,
            current_column: 1,
            last_newline_pos: 0,
        }
    }

    /// Get the next token with position information
    pub fn next_token(&mut self) -> Option<Result<LocatedToken, LocatedToken>> {
        let token_result = self.lexer.next()?;
        let span = self.lexer.span();
        let slice = self.lexer.slice();

        let start_pos = self.calculate_position(span.start);

        // Update position tracking
        self.update_position_for_slice(slice);

        let end_pos = self.calculate_position(span.end);

        match token_result {
            Ok(token) => {
                let located_token = LocatedToken::new(token, span, start_pos, end_pos);
                Some(Ok(located_token))
            }
            Err(_) => {
                let error_token = LocatedToken::new(Token::Error, span, start_pos, end_pos);
                Some(Err(error_token))
            }
        }
    }

    /// Peek at the next token without consuming it
    pub fn peek(&self) -> Option<Result<Token, Token>> {
        let mut clone_lexer = self.lexer.clone();
        match clone_lexer.next()? {
            Ok(token) => Some(Ok(token)),
            Err(_) => Some(Err(Token::Error)),
        }
    }

    /// Get the current span in the source
    pub fn span(&self) -> Range<usize> {
        self.lexer.span()
    }

    /// Get the current slice of text
    pub fn slice(&self) -> &'a str {
        self.lexer.slice()
    }

    /// Get the remaining source code
    pub fn remainder(&self) -> &'a str {
        self.lexer.remainder()
    }

    /// Get all tokens from the source (useful for debugging)
    pub fn tokenize_all(source: &'a str) -> Vec<Result<LocatedToken, LocatedToken>> {
        let mut lexer = Self::new(source);
        let mut tokens = Vec::new();

        while let Some(token_result) = lexer.next_token() {
            tokens.push(token_result);
        }

        tokens
    }

    /// Filter out comments and return only code tokens
    pub fn tokenize_code_only(source: &'a str) -> Vec<Result<LocatedToken, LocatedToken>> {
        Self::tokenize_all(source)
            .into_iter()
            .filter(|token_result| {
                match token_result {
                    Ok(located_token) => !located_token.token.is_comment(),
                    Err(_) => true, // Keep errors
                }
            })
            .collect()
    }

    /// Check if we're at the end of the source
    pub fn is_at_end(&self) -> bool {
        self.lexer.remainder().is_empty()
    }

    /// Get current position
    pub fn current_position(&self) -> Position {
        Position::new(
            self.current_line,
            self.current_column,
            self.lexer.span().start,
        )
    }

    /// Calculate position from byte offset
    fn calculate_position(&self, offset: usize) -> Position {
        let mut line = 1;
        let mut column = 1;

        for (i, ch) in self.source.char_indices() {
            if i >= offset {
                break;
            }

            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        Position::new(line, column, offset)
    }

    /// Update internal position tracking based on consumed slice
    fn update_position_for_slice(&mut self, slice: &str) {
        for ch in slice.chars() {
            if ch == '\n' {
                self.current_line += 1;
                self.current_column = 1;
                self.last_newline_pos = self.lexer.span().end;
            } else {
                self.current_column += 1;
            }
        }
    }
}

/// Iterator implementation for the lexer
impl<'a> Iterator for WidowLexer<'a> {
    type Item = Result<LocatedToken, LocatedToken>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

/// Utility functions for working with tokens
impl Token {
    /// Check if this token should be ignored during parsing (whitespace, comments)
    pub fn should_ignore_for_parsing(&self) -> bool {
        matches!(self, Token::LineComment(_) | Token::BlockComment(_))
    }

    /// Check if this token is significant for parsing
    pub fn is_significant(&self) -> bool {
        !self.should_ignore_for_parsing()
    }

    /// Get the binding power for operators (used in precedence parsing)
    pub fn binding_power(&self) -> Option<(u8, u8)> {
        match self {
            // Assignment operators (right associative)
            Token::Assign
            | Token::PlusAssign
            | Token::MinusAssign
            | Token::MultiplyAssign
            | Token::DivideAssign
            | Token::ModuloAssign => Some((2, 1)),

            // Logical OR
            Token::Or => Some((3, 4)),

            // Logical AND
            Token::And => Some((5, 6)),

            // Equality
            Token::Equal | Token::NotEqual => Some((7, 8)),

            // Comparison
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual => Some((9, 10)),

            // Bitwise OR
            Token::BitwiseOr => Some((11, 12)),

            // Bitwise XOR
            Token::BitwiseXor => Some((13, 14)),

            // Bitwise AND
            Token::BitwiseAnd => Some((15, 16)),

            // Shift operators
            Token::LeftShift | Token::RightShift => Some((17, 18)),

            // Addition/Subtraction
            Token::Plus | Token::Minus => Some((19, 20)),

            // Multiplication/Division/Modulo
            Token::Multiply | Token::Divide | Token::Modulo => Some((21, 22)),

            // Power (right associative)
            Token::Power => Some((24, 23)),

            // Member access and method calls (left associative, highest precedence)
            Token::Dot | Token::SafeAccess => Some((25, 26)),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let source = "func add(a, b) { ret a + b }";
        let mut lexer = WidowLexer::new(source);

        let tokens: Vec<_> = lexer.collect();
        assert!(!tokens.is_empty());

        // Check first token
        if let Some(Ok(first_token)) = tokens.first() {
            assert_eq!(first_token.token, Token::Func);
            assert_eq!(first_token.span, 0..4);
        }
    }

    #[test]
    fn test_position_tracking() {
        let source = "line1\nline2\nline3";
        let mut lexer = WidowLexer::new(source);

        // Should handle newlines correctly
        while let Some(token_result) = lexer.next_token() {
            if let Ok(located_token) = token_result {
                if located_token.token == Token::Newline {
                    assert!(located_token.start_pos.line <= located_token.end_pos.line);
                }
            }
        }
    }

    #[test]
    fn test_string_literals() {
        let source = r#""hello world" r"raw string" `template ${var}`"#;
        let tokens = WidowLexer::tokenize_all(source);

        let mut string_count = 0;
        for token_result in tokens {
            if let Ok(located_token) = token_result {
                match located_token.token {
                    Token::String(_) | Token::RawString(_) | Token::TemplateString(_) => {
                        string_count += 1;
                    }
                    _ => {}
                }
            }
        }
        assert_eq!(string_count, 3);
    }

    #[test]
    fn test_numeric_literals() {
        let source = "42 3.14 123.456e10";
        let tokens = WidowLexer::tokenize_all(source);

        let mut numeric_count = 0;
        for token_result in tokens {
            if let Ok(located_token) = token_result {
                match located_token.token {
                    Token::Integer(_) | Token::Float(_) => {
                        numeric_count += 1;
                    }
                    _ => {}
                }
            }
        }
        assert_eq!(numeric_count, 3);
    }

    #[test]
    fn test_comments() {
        // Test line comment
        let line_comment_test = "// This is a line comment";
        let tokens = WidowLexer::tokenize_all(line_comment_test);
        assert!(
            tokens.iter().any(
                |t| matches!(t, Ok(located) if matches!(located.token, Token::LineComment(_)))
            )
        );

        // Test TODO comment (treated as regular line comment)
        let todo_comment_test = "// TODO: Fix this issue";
        let tokens = WidowLexer::tokenize_all(todo_comment_test);
        assert!(
            tokens.iter().any(
                |t| matches!(t, Ok(located) if matches!(located.token, Token::LineComment(_)))
            )
        );

        // Test doc comment (single line)
        let doc_comment_test = "/** This is a doc comment **/";
        let tokens = WidowLexer::tokenize_all(doc_comment_test);
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Ok(located) if matches!(located.token, Token::DocComment(_))))
        );

        // Test simple source with comments
        let source_with_comments = r#"
// Simple line comment
/** Simple doc comment **/
// TODO: Add feature
"#;

        let tokens = WidowLexer::tokenize_all(source_with_comments);
        let comment_count = tokens
            .iter()
            .filter(|t| matches!(t, Ok(located) if located.token.is_comment()))
            .count();

        assert!(comment_count >= 3, "Should have at least 3 comments");
    }
}
