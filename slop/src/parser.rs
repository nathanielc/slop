use std::{fmt::Display, iter::Peekable, str::CharIndices};

use thiserror::Error;

use crate::ast::{self, Position, Positioned, Quantity};

#[derive(Debug)]
pub enum Token<'input> {
    OpenAngle,
    CloseAngle,
    Equal,
    Hash,
    Colon,
    Star,
    Hat,
    StarStar,
    HashStar,
    HashHash,
    Number(&'input str),
    Fraction(&'input str),
    Sentence(&'input str),
}

impl<'input> Display for Token<'input> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::OpenAngle => write!(f, "<"),
            Token::CloseAngle => write!(f, ">"),
            Token::Equal => write!(f, "="),
            Token::Hash => write!(f, "#"),
            Token::Colon => write!(f, ":"),
            Token::Star => write!(f, "*"),
            Token::Hat => write!(f, "^"),
            Token::StarStar => write!(f, "**"),
            Token::HashStar => write!(f, "#*"),
            Token::HashHash => write!(f, "##"),
            Token::Number(s) => write!(f, "{s}"),
            Token::Fraction(s) => write!(f, "{s}"),
            Token::Sentence(s) => write!(f, "{s}"),
        }
    }
}
struct Lexer<'input> {
    input: &'input str,
    iter: Peekable<CharIndices<'input>>,
}
impl<'input> Lexer<'input> {
    fn new(input: &'input str) -> Self {
        Self {
            input,
            iter: input.char_indices().peekable(),
        }
    }
    fn skip_whitespace(&mut self) {
        while let Some((_, ch)) = self.iter.peek() {
            if ch.is_whitespace() {
                self.iter.next();
            } else {
                return;
            }
        }
    }
    fn lex_sentence(&mut self, start: usize) -> (Token<'input>, Position) {
        while let Some((end, ch)) = self.iter.peek() {
            if is_sentence_char(*ch) {
                self.iter.next();
            } else {
                return (Token::Sentence(self.input[start..*end].trim()), start..*end);
            };
        }
        (
            Token::Sentence(&self.input[start..]),
            start..self.input.len(),
        )
    }
    fn lex_number_or_fraction(&mut self, start: usize) -> (Token<'input>, Position) {
        while let Some((end, ch)) = self.iter.peek() {
            let end = *end;
            match ch {
                '/' => {
                    self.iter.next();
                    return (Token::Fraction(self.lex_digit(start)), start..end);
                }
                '.' => {
                    self.iter.next();
                    return (Token::Number(self.lex_digit(start)), start..end);
                }
                n if n.is_numeric() => {
                    self.iter.next();
                }
                _ => {
                    return (Token::Number(&self.input[start..end]), start..end);
                }
            }
        }
        (Token::Number(&self.input[start..]), start..self.input.len())
    }
    fn lex_digit(&mut self, start: usize) -> &'input str {
        while let Some((stop, ch)) = self.iter.peek() {
            if ch.is_numeric() {
                self.iter.next()
            } else {
                return &self.input[start..*stop];
            };
        }
        &self.input[start..]
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = (Token<'input>, Position);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some((start, '<')) => return Some((Token::OpenAngle, start..start)),
                Some((start, '>')) => return Some((Token::CloseAngle, start..start)),
                Some((start, '=')) => return Some((Token::Equal, start..start)),
                Some((start, '#')) => {
                    return match self.iter.peek() {
                        Some((end, '*')) => {
                            let end = *end;
                            self.iter.next();
                            Some((Token::HashStar, start..end))
                        }
                        Some((end, '#')) => {
                            let end = *end;
                            self.iter.next();
                            Some((Token::HashHash, start..end))
                        }
                        Some(_) => Some((Token::Hash, start..start)),
                        None => None,
                    }
                }
                Some((start, ':')) => return Some((Token::Colon, start..start)),
                Some((start, '*')) => {
                    return match self.iter.peek() {
                        Some((end, '*')) => {
                            let end = *end;
                            self.iter.next();
                            Some((Token::StarStar, start..end))
                        }
                        Some(_) => Some((Token::Star, start..start)),
                        None => None,
                    }
                }
                Some((start, '^')) => return Some((Token::Hat, start..start)),
                Some((start, ch)) if ch.is_numeric() => {
                    return Some(self.lex_number_or_fraction(start))
                }
                Some((_, ch)) if ch.is_whitespace() => {
                    self.skip_whitespace();
                }
                Some((start, _)) => return Some(self.lex_sentence(start)),
                None => return None,
            }
        }
    }
}

fn is_sentence_char(ch: char) -> bool {
    ch != '*' && ch != '#' && ch != '=' && ch != '>' && ch != ':'
}

pub fn parse(input: &str) -> (ast::SourceFile, Vec<Error>) {
    let mut parser = Parser {
        lexer: Lexer::new(input).peekable(),
        stack: Default::default(),
        errors: Default::default(),
    };
    let src = parser.parse_source_file();
    (src, parser.errors)
}

// Handwritten parser so we can handle missing and unused operators.
//
// The parser is a greedy stack parser. It pushes each operand onto a stack when its encountered
// and pops items off the stack for each operator. When the stack does not contain enough operands
// [`Operand::MissingOperand`] are created to fill the need. When the stack contains extra operands
// [`Operand::UnusedOperands`] is created to consume them.
struct Parser<'input> {
    lexer: Peekable<Lexer<'input>>,
    stack: Vec<ast::Operand>,
    errors: Vec<Error>,
}
impl<'input> Parser<'input> {
    fn unexpected(&mut self, token: Option<(Token<'input>, Position)>) -> Position {
        let (error, position) = if let Some((token, position)) = token {
            (
                Error::UnexpectedToken(token.to_string(), position.clone()),
                position,
            )
        } else {
            (Error::UnexpectedEOF, Position::default())
        };
        self.errors.push(error);
        position
    }
    fn expect_text(&mut self) -> (String, Position) {
        match self.lexer.next() {
            Some((Token::Sentence(text), position)) => (text.to_string(), position),
            t => ("".to_string(), self.unexpected(t)),
        }
    }
    fn parse_source_file(&mut self) -> ast::SourceFile {
        let mut recipes = Vec::new();
        while let Some((Token::OpenAngle, _)) = self.lexer.peek() {
            recipes.push(self.parse_recipe())
        }
        ast::SourceFile { recipes }
    }
    fn parse_recipe(&mut self) -> ast::Recipe {
        let start = match self.lexer.next() {
            Some((Token::OpenAngle, position)) => position,
            t => self.unexpected(t),
        };
        let title = match self.lexer.peek() {
            Some((Token::StarStar, _)) => {
                self.lexer.next();
                match self.lexer.next() {
                    Some((Token::Sentence(title), _)) => Some(title.to_string()),
                    t => {
                        self.unexpected(t);
                        None
                    }
                }
            }
            _ => None,
        };
        let preamble = match self.lexer.peek() {
            Some((Token::HashHash, _)) => {
                self.lexer.next();
                match self.lexer.next() {
                    Some((Token::Sentence(preamble), _)) => Some(preamble.to_string()),
                    t => {
                        self.unexpected(t);
                        None
                    }
                }
            }
            _ => None,
        };
        self.parse_operands();
        let root = match self.stack.len() {
            1 => self.stack.pop().unwrap(),
            l if l > 1 => {
                let operands: Vec<ast::Operand> = self.stack.drain(..).collect();
                let start = operands[0].position().start;
                let end = operands.iter().last().unwrap().position().end;
                ast::Operand::UnusedOperands {
                    position: start..end,
                    operands,
                }
            }
            _ => ast::Operand::MissingOperand {
                // TODO determine position of missing operand
                position: Default::default(),
            },
        };
        debug_assert!(self.stack.is_empty());
        let comment = match self.lexer.peek() {
            Some((Token::HashStar, _)) => {
                self.lexer.next();
                match self.lexer.next() {
                    Some((Token::Sentence(comment), _)) => Some(comment.to_string()),
                    t => {
                        self.unexpected(t);
                        None
                    }
                }
            }
            _ => None,
        };
        let end = match self.lexer.next() {
            Some((Token::CloseAngle, position)) => position,
            t => self.unexpected(t),
        };
        ast::Recipe {
            position: start.start..end.end,
            title,
            preamble,
            comment,
            root,
        }
    }
    fn parse_operands(&mut self) {
        while let Some((Token::Star, _)) | Some((Token::Equal, _)) | Some((Token::Hash, _)) =
            self.lexer.peek()
        {
            self.parse_operand()
        }
    }
    fn parse_operand(&mut self) {
        match self.lexer.next() {
            Some((Token::Star, start)) => {
                let derived = match self.lexer.peek() {
                    Some((Token::Hat, _)) => {
                        self.lexer.next();
                        true
                    }
                    _ => false,
                };
                let mut quantities = Vec::new();
                loop {
                    match self.lexer.peek() {
                        Some((Token::Number(number), _)) => {
                            quantities.push(Quantity::Number(number.to_string()));
                            self.lexer.next();
                        }
                        Some((Token::Fraction(fraction), _)) => {
                            quantities.push(Quantity::Fraction(fraction.to_string()));
                            self.lexer.next();
                        }
                        _ => break,
                    };
                }
                let unit = if !quantities.is_empty() {
                    let unit = match self.lexer.peek() {
                        Some((Token::Sentence(unit), _)) => {
                            let unit = unit.to_string();
                            self.lexer.next();
                            Some(unit)
                        }
                        _ => None,
                    };
                    match self.lexer.next() {
                        Some((Token::Colon, _)) => {}
                        t => {
                            self.unexpected(t);
                        }
                    };
                    unit
                } else {
                    None
                };
                let (text, end) = self.expect_text();
                self.stack.push(ast::Operand::Ingredient {
                    position: start.start..end.end,
                    derived,
                    quantities,
                    unit,
                    text,
                });
            }
            Some((Token::Equal, start)) => {
                let operand = self.stack.pop().unwrap_or(ast::Operand::MissingOperand {
                    position: start.clone(),
                });
                let (text, end) = self.expect_text();
                self.stack.push(ast::Operand::UnaryOp {
                    position: start.start..end.end,
                    operand: Box::new(operand),
                    text,
                });
            }
            Some((Token::Hash, start)) => {
                let second = self.stack.pop().unwrap_or(ast::Operand::MissingOperand {
                    position: start.clone(),
                });
                let first = self.stack.pop().unwrap_or(ast::Operand::MissingOperand {
                    position: start.clone(),
                });
                let (text, end) = self.expect_text();
                self.stack.push(ast::Operand::BinaryOp {
                    position: start.start..end.end,
                    first: Box::new(first),
                    second: Box::new(second),
                    text,
                });
            }
            t => {
                self.unexpected(t);
            }
        };
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected token: {0}")]
    UnexpectedToken(String, Position),
    #[error("unexpected end of input")]
    UnexpectedEOF,
}
