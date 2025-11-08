use crate::token::Token;

/// Character-by-character lexical analyzer facade (per project spec).
#[allow(dead_code)]
pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

/// Lexer state: input as chars, current position, and a scratch buffer.
pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    buf: String, // scratch for current lexeme
}

impl Lexer {
    /* ---------- ctor ---------- */

    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            buf: String::new(),
        }
    }

    /* ---------- basic cursor ops ---------- */

    #[inline]
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    #[inline]
    fn advance(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            return None;
        }
        let c = self.chars[self.pos];
        self.pos += 1;
        Some(c)
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if matches!(c, ' ' | '\t' | '\n' | '\r') {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Uppercased lookahead for tag detection without consuming.
    #[allow(dead_code)]
    fn lookahead_upper(&self, n: usize) -> String {
        let mut s: String = self.chars.iter().skip(self.pos).take(n).collect();
        s.make_ascii_uppercase();
        s
    }

    /* ---------- lexeme readers ---------- */

    /// Read a bare “word” used for keywords/identifiers/URLs.
    fn read_word(&mut self) -> String {
        self.buf.clear();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || matches!(c, '/' | ':' | '.' | '_') {
                self.buf.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.buf.clone()
    }

    /// Read the alphabetical word that follows '#', returned in UPPERCASE.
    /// (No spaces here — multi-word tags are handled explicitly.)
    fn read_tag_word_upper(&mut self) -> String {
        self.buf.clear();
        while let Some(c) = self.peek() {
            if c.is_alphabetic() {
                self.buf.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let mut up = self.buf.clone();
        up.make_ascii_uppercase();
        up
    }

    /// Read text content until we’d start a new tag, preserving newlines.
    fn read_until_mkay_or_eol(&mut self) -> String {
        let mut out = String::new();
        while let Some(c) = self.peek() {
            if matches!(c, '\n' | '\r') {
                out.push(c);
                self.advance();
                continue;
            }
            if c == '#' {
                // Stop here; let next_token() lex the tag (#MKAY, #KTHXBYE, #LEMME SEE, etc.)
                break;
            }
            out.push(c);
            self.advance();
        }
        out.trim().to_string()
    }

    /* ---------- public tokenization ---------- */

    pub fn next_token(&mut self) -> Token {
        self.skip_ws();
        let Some(c0) = self.peek() else { return Token::Eof; };

        // TAGS that start with '#'
        if c0 == '#' {
            self.advance(); // consume '#'

            // Read the first tag word (letters only)
            let w1 = self.read_tag_word_upper();

            // Multi-word tags: "#I HAZ", "#IT IZ", "#LEMME SEE"
            if w1 == "I" {
                self.skip_ws();
                let w2 = self.read_tag_word_upper();
                if w2 == "HAZ" { return Token::HashIHaz; }
                eprintln!("Lexical error: expected 'HAZ' after '#I'.");
                std::process::exit(1);
            }
            if w1 == "IT" {
                self.skip_ws();
                let w2 = self.read_tag_word_upper();
                if w2 == "IZ" { return Token::HashItIz; }
                eprintln!("Lexical error: expected 'IZ' after '#IT'.");
                std::process::exit(1);
            }
            if w1 == "LEMME" {
                self.skip_ws();
                let w2 = self.read_tag_word_upper();
                if w2 == "SEE" { return Token::HashLemmeSee; }
                eprintln!("Lexical error: expected 'SEE' after '#LEMME'.");
                std::process::exit(1);
            }

            // Single-word tags
            return match w1.as_str() {
                "HAI"      => Token::HashHai,
                "KTHXBYE"  => Token::HashKthxbye,
                "OBTW"     => Token::HashObtW,
                "TLDR"     => Token::HashTldr,
                "MAEK"     => Token::HashMaek,
                "OIC"      => Token::HashOic,
                "GIMMEH"   => Token::HashGimmeh,
                "MKAY"     => Token::HashMKay,
                _ => {
                    eprintln!("Lexical error: unknown tag '#{}'", w1);
                    std::process::exit(1);
                }
            };
        }

        // Bare keywords / identifiers
        if c0.is_alphabetic() {
            let w = self.read_word();
            let mut up = w.clone();
            up.make_ascii_uppercase();
            return match up.as_str() {
                "HEAD"     => Token::Head,
                "TITLE"    => Token::Title,
                "PARAGRAF" => Token::Paragraf,
                "BOLD"     => Token::Bold,
                "ITALICS"  => Token::Italics,
                "LIST"     => Token::List,
                "ITEM"     => Token::Item,
                "NEWLINE"  => Token::Newline,
                "SOUNDZ"   => Token::Soundz,
                "VIDZ"     => Token::Vidz,
                _ => Token::Text(w),
            };
        }

        // Otherwise: free text until next control
        let text = self.read_until_mkay_or_eol();
        if text.is_empty() {
            Token::Eof
        } else {
            Token::Text(text)
        }
    }
}

/* ---------- trait facade impl ---------- */

impl LexicalAnalyzer for Lexer {
    fn get_char(&mut self) -> char {
        self.advance().unwrap_or_else(|| {
            eprintln!("Unexpected end of input while lexing");
            std::process::exit(1);
        })
    }

    fn add_char(&mut self, c: char) {
        self.buf.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
        let mut up = s.to_string();
        up.make_ascii_uppercase();
        matches!(
            up.as_str(),
            "#HAI"
                | "#KTHXBYE"
                | "#OBTW"
                | "#TLDR"
                | "#MAEK"
                | "#OIC"
                | "#GIMMEH"
                | "#MKAY"
                | "HEAD"
                | "TITLE"
                | "PARAGRAF"
                | "BOLD"
                | "ITALICS"
                | "LIST"
                | "ITEM"
                | "NEWLINE"
                | "SOUNDZ"
                | "VIDZ"
                | "#I HAZ"
                | "#IT IZ"
                | "#LEMME SEE"
        )
    }
}
