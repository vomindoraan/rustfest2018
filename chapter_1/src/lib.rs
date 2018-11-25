use std::str::Bytes;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    /// Should match a combination of letters a-z and A-Z.
    /// We don't need underscores but feel free to add them.
    Identifier,

    /// We will just need integers, so sequences of digits 0-9
    /// will suffice
    Number,

    /// '+'
    Add,

    /// '-'
    Subtract,

    /// '*'
    Multiply,

    /// '/'
    Divide,

    /// '='
    Assign,

    /// ';'
    Semicolon,
}

/// This struct needs some fields!
#[derive(Debug)]
pub struct Lexer<It: Iterator<Item=u8>> {
    source: It,
}

impl<'a> Lexer<Bytes<'a>> {
    /// It also needs some code
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source: source.bytes(),
        }
    }
}

/// We will also use the `Iterator` trait from the
/// standard library for our Lexer.
impl<It: Iterator<Item = u8>> Iterator for Lexer<It> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            match self.source.next()? {
                b'a'...b'z' | b'A'...b'Z' | b'_' => {
                    break Some(Token::Identifier)
                },
                b'0'...b'9' => {
                    break Some(Token::Number)
                },
                b'+' => break Some(Token::Add),
                b'-' => break Some(Token::Subtract),
                b'*' => break Some(Token::Multiply),
                b'/' => break Some(Token::Divide),
                b'=' => break Some(Token::Assign),
                b';' => break Some(Token::Semicolon),
                b' ' | b'\t' | b'\n' => continue,
                _ => break None,
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    let source = "four = 2 + 2; omg = 12345 / 0;";

    let expect = &[
        Token::Identifier,
        Token::Assign,
        Token::Number,
        Token::Add,
        Token::Number,
        Token::Semicolon,
        Token::Identifier,
        Token::Assign,
        Token::Number,
        Token::Divide,
        Token::Number,
        Token::Semicolon,
    ];

    // Create an iterator of Tokens out of the slice here.
    let expect = expect.iter().cloned();
    let lexer = Lexer::new(source);
    println!("{:?}", lexer);

    // We can use the `eq` method of the `Iterator` trait
    // to check that they are equal
    assert!(lexer.eq(expect));
}
