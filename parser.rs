// src/parser.rs
use crate::lexer::Lexer;
use crate::scope::Scope;
use crate::token::Token;
use crate::htmlgen::Html;

// Trait signatures from the project handout (Option 1)
#[allow(dead_code)]
pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self);
    fn parse_head(&mut self);
    fn parse_title(&mut self);
    fn parse_comment(&mut self);
    fn parse_body(&mut self);
    fn parse_paragraph(&mut self);
    fn parse_inner_paragraph(&mut self);
    fn parse_inner_text(&mut self);
    fn parse_variable_define(&mut self);
    fn parse_variable_use(&mut self);
    fn parse_bold(&mut self);
    fn parse_italics(&mut self);
    fn parse_list(&mut self);
    fn parse_list_items(&mut self);
    fn parse_inner_list(&mut self);
    fn parse_audio(&mut self);
    fn parse_video(&mut self);
    fn parse_newline(&mut self);
    fn parse_text(&mut self);
}

pub struct Parser<'a> {
    lexer: Lexer,
    look: Token,
    html: Html,
    scope: Scope,
    _source_name: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str, source_name: &'a str) -> Self {
        let mut lex = Lexer::new(input);
        let look = lex.next_token();
        Self {
            lexer: lex,
            look,
            html: Html::default(),
            scope: Scope::new(),
            _source_name: source_name,
        }
    }

    fn eat(&mut self, expected: Token) {
        if std::mem::discriminant(&self.look) == std::mem::discriminant(&expected) {
            self.look = self.lexer.next_token();
        } else {
            eprintln!("Syntax error: expected {:?}, found {:?}", expected, self.look);
            std::process::exit(1);
        }
    }

    // (Now unused, keep for spec parity; silence warning if you want to keep it)
    #[allow(dead_code)]
    fn eat_any_text(&mut self) -> String {
        if let Token::Text(t) = self.look.clone() {
            self.look = self.lexer.next_token();
            t
        } else {
            eprintln!("Syntax error: expected TEXT, found {:?}", self.look);
            std::process::exit(1);
        }
    }

    /// Helper: collect TEXT tokens until we hit #MKAY, then consume #MKAY.
    fn read_text_until_mkay(&mut self) -> String {
        let mut out = String::new();
        while !matches!(self.look, Token::HashMKay | Token::Eof) {
            match self.look.clone() {
                Token::Text(t) => {
                    if !out.is_empty() { out.push(' '); }
                    out.push_str(&t);
                    self.look = self.lexer.next_token();
                }
                _ => {
                    eprintln!("Syntax error: only text is allowed before #MKAY here");
                    std::process::exit(1);
                }
            }
        }
        self.eat(Token::HashMKay);
        out
    }

    /// Helper: parse “rich” inline content (text, bold, italics, newline, variables, comments)
    /// until we reach a terminating `#MKAY`. Consumes that `#MKAY`.
    fn parse_inline_until_mkay(&mut self) {
        while !matches!(self.look, Token::HashMKay | Token::Eof) {
            match self.look {
                Token::HashGimmeh => {
                    self.eat(Token::HashGimmeh);
                    match self.look {
                        Token::Bold    => self.parse_bold(),
                        Token::Italics => self.parse_italics(),
                        Token::Newline => self.parse_newline(),
                        _ => {
                            eprintln!("Unsupported #GIMMEH construct inside this block");
                            std::process::exit(1);
                        }
                    }
                }
                Token::HashLemmeSee => self.parse_variable_use(),
                Token::Text(_) => self.parse_text(),
                Token::HashObtW => self.parse_comment(),
                Token::HashMKay => break, // stop condition
                _ => {
                    eprintln!("Unexpected token inside inline block: {:?}", self.look);
                    std::process::exit(1);
                }
            }
        }
        self.eat(Token::HashMKay);
    }

    pub fn into_html(self) -> String { self.html.finish() }
}

impl<'a> SyntaxAnalyzer for Parser<'a> {
    fn parse_lolcode(&mut self) {
    // <!doctype html><html>
    self.html.begin_html();

    // Must start with #HAI
    match self.look {
        Token::HashHai => self.eat(Token::HashHai),
        _ => {
            eprintln!("Program must start with #HAI");
            std::process::exit(1);
        }
    }

    // zero or more comments
    while matches!(self.look, Token::HashObtW) {
        self.parse_comment();
    }

    // optional head: #MAEK HEAD ... #OIC
    if matches!(self.look, Token::HashMaek) {
        let _save = self.look.clone();
        self.eat(Token::HashMaek);
        if matches!(self.look, Token::Head) {
            self.parse_head();
        } else {
            eprintln!("Syntax error: after #MAEK expected HEAD or PARAGRAF");
            std::process::exit(1);
        }
    }

    // Ensure <body> is open so plain text lands inside it
    self.html.begin_body();

    // body: zero or more constructs until #KTHXBYE
    while !matches!(self.look, Token::HashKthxbye | Token::Eof) {
        self.parse_body();
    }

    // Must end with #KTHXBYE
    self.eat(Token::HashKthxbye);

    // Close </body> and then </html>
    self.html.end_body();

    // NOTE:
    // If your htmlgen.finish() already appends </html>, you can remove the next line.
    // Keep it ONLY if your Html.finish() does NOT auto-close.
    self.html.end_html();
}


    fn parse_head(&mut self) {
        // we’ve already consumed #MAEK then saw HEAD
        self.eat(Token::Head);
        self.html.begin_head();
        self.parse_title();
        self.eat(Token::HashOic);
        self.html.end_head();
    }

    fn parse_title(&mut self) {
        self.eat(Token::HashGimmeh);
        self.eat(Token::Title);
        let t = self.read_text_until_mkay(); // collect "The Simpsons" etc.
        self.html.title(&t);
    }

    fn parse_comment(&mut self) {
        self.eat(Token::HashObtW);
        // accumulate everything until #TLDR
        let mut text = String::new();
        while !matches!(self.look, Token::HashTldr | Token::Eof) {
            match self.look.clone() {
                Token::Text(t) => {
                    if !text.is_empty() { text.push(' '); }
                    text.push_str(&t);
                    self.look = self.lexer.next_token();
                }
                _ => {
                    eprintln!("Syntax error: only text is allowed inside #OBTW ... #TLDR comments");
                    std::process::exit(1);
                }
            }
        }
        self.eat(Token::HashTldr);
        self.html.comment(&text);
    }


    fn parse_body(&mut self) {
        match self.look {
            Token::HashMaek => {
                self.eat(Token::HashMaek);
                match self.look {
                    Token::Paragraf => self.parse_paragraph(),
                    Token::List     => self.parse_list(),   // NEW: lists
                    _ => { eprintln!("After #MAEK expected PARAGRAF or LIST"); std::process::exit(1); }
                }
            }
            Token::HashGimmeh => {
                self.eat(Token::HashGimmeh);
                match self.look {
                    Token::Newline => self.parse_newline(),
                    Token::Bold    => self.parse_bold(),
                    Token::Italics => self.parse_italics(),
                    Token::Soundz  => self.parse_audio(),   // NEW: sound
                    Token::Vidz    => self.parse_video(),   // NEW: video
                    _ => { eprintln!("Unsupported/Unexpected #GIMMEH construct in body"); std::process::exit(1); }
                }
            }
            Token::HashIHaz => self.parse_variable_define(),
            Token::HashLemmeSee => self.parse_variable_use(),
            Token::Text(_) => self.parse_text(),
            Token::HashObtW => self.parse_comment(),

            // tolerate stray #MKAY at top level
            Token::HashMKay => { self.eat(Token::HashMKay); }

            _ => { eprintln!("Unexpected token in body: {:?}", self.look); std::process::exit(1); }
        }
    }

    fn parse_paragraph(&mut self) {
        self.eat(Token::Paragraf);
        self.scope.push(); // new block scope
        self.html.begin_p();

        // Optional immediate var define (per spec)
        if matches!(self.look, Token::HashIHaz) {
            self.parse_variable_define();
        }

        // Inner paragraph content
        self.parse_inner_paragraph();

        self.eat(Token::HashOic);
        self.html.end_p();
        self.scope.pop();
    }

    fn parse_inner_paragraph(&mut self) {
        // zero or more inner-text elements until #OIC
        while !matches!(self.look, Token::HashOic | Token::Eof) {
            self.parse_inner_text();
        }
    }

    fn parse_inner_text(&mut self) {
        match self.look {
            Token::HashGimmeh => {
                self.eat(Token::HashGimmeh);
                match self.look {
                    Token::Bold    => self.parse_bold(),
                    Token::Italics => self.parse_italics(),
                    Token::Newline => self.parse_newline(),
                    _ => { eprintln!("Unsupported #GIMMEH in paragraph"); std::process::exit(1); }
                }
            }
            Token::HashLemmeSee => self.parse_variable_use(),
            Token::Text(_) => self.parse_text(),
            Token::HashObtW => self.parse_comment(),
            // tolerate a stray #MKAY inside a paragraph (consume and continue)
            Token::HashMKay => { self.eat(Token::HashMKay); }
            _ => { eprintln!("Unexpected token in paragraph: {:?}", self.look); std::process::exit(1); }
        }
    }

    fn parse_variable_define(&mut self) {
        self.eat(Token::HashIHaz);
        let name = if let Token::Text(t) = self.look.clone() {
            self.eat(Token::Text(t.clone()));
            t
        } else {
            eprintln!("Expected variable name after #I HAZ");
            std::process::exit(1);
        };
        self.eat(Token::HashItIz);
        let val = self.read_text_until_mkay();  // ✅ collects entire value before #MKAY
        self.scope.define(&name, val);
    }


    fn parse_variable_use(&mut self) {
        // #LEMME SEE <name> #MKAY
        self.eat(Token::HashLemmeSee);
        let name = if let Token::Text(t) = self.look.clone() {
            self.eat(Token::Text(t.clone()));
            t
        } else {
            eprintln!("Expected variable name after #LEMME SEE");
            std::process::exit(1);
        };
        self.eat(Token::HashMKay);
        if let Some(v) = self.scope.resolve(&name) {
            self.html.push(&v);
        } else {
            eprintln!("Static semantic error: variable '{}' used before definition", name);
            std::process::exit(1);
        }
    }

    fn parse_bold(&mut self) {
        self.eat(Token::Bold);
        let t = self.read_text_until_mkay(); // collect multi-word bold text
        self.html.bold(&t);
    }

    fn parse_italics(&mut self) {
        self.eat(Token::Italics);
        let t = self.read_text_until_mkay(); // collect multi-word italics text
        self.html.italics(&t);
    }

    /* ===================== NEW: LIST / ITEM ===================== */

    fn parse_list(&mut self) {
        // We are after #MAEK; current token is LIST
        self.eat(Token::List);
        self.html.push("<ul>");

        // Inside a list, expect zero or more "#GIMMEH ITEM ... #MKAY"
        loop {
            match self.look {
                Token::HashGimmeh => {
                    self.eat(Token::HashGimmeh);
                    match self.look {
                        Token::Item => self.parse_list_items(),
                        Token::Newline => self.parse_newline(), // allow line breaks in list body
                        _ => {
                            eprintln!("Inside LIST: expected ITEM after #GIMMEH");
                            std::process::exit(1);
                        }
                    }
                }
                Token::HashObtW => self.parse_comment(),
                Token::HashOic => {
                    self.eat(Token::HashOic);
                    break;
                }
                Token::Eof => {
                    eprintln!("Syntax error: unexpected EOF inside LIST");
                    std::process::exit(1);
                }
                _ => {
                    // Be strict: only items/comments/newlines allowed inside a LIST
                    eprintln!("Unexpected token inside LIST: {:?}", self.look);
                    std::process::exit(1);
                }
            }
        }

        self.html.push("</ul>");
    }

    fn parse_list_items(&mut self) {
        // Current token is ITEM
        self.eat(Token::Item);
        self.html.push("<li>");
        self.parse_inner_list(); // parses until #MKAY, supporting rich inline content
        self.html.push("</li>");
    }

    fn parse_inner_list(&mut self) {
        // Parse inline constructs until we hit #MKAY (end of ITEM)
        self.parse_inline_until_mkay();
    }

    /* ===================== NEW: AUDIO / VIDEO ===================== */

    fn parse_audio(&mut self) {
        self.eat(Token::Soundz);
        let src = self.read_text_until_mkay();
        let src = src.trim();
        self.html.push(format!(
            "<audio controls><source src=\"{}\" /></audio>",
            html_escape(src)
        ));
    }

    fn parse_video(&mut self) {
        self.eat(Token::Vidz);
        let src = self.read_text_until_mkay();
        let src = src.trim();
        // Simple iframe; you can style/size later as needed
        self.html.push(format!(
            "<iframe src=\"{}\" allowfullscreen loading=\"lazy\"></iframe>",
            html_escape(src)
        ));
    }

    fn parse_newline(&mut self) {
        self.eat(Token::Newline);
        self.eat(Token::HashMKay);
        self.html.br();
    }

    fn parse_text(&mut self) {
        if let Token::Text(t) = self.look.clone() {
            self.eat(Token::Text(t.clone()));
            self.html.text(&t);
        } else {
            eprintln!("Internal: parse_text called on non-text");
            std::process::exit(1);
        }
    }
}

/* ---------- tiny HTML escaper for attributes ---------- */
fn html_escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            _ => c.to_string(),
        })
        .collect::<String>()
}

pub struct FrontEnd<'a> {
    parser: Parser<'a>,
}

impl<'a> FrontEnd<'a> {
    pub fn new(input: &str, source_name: &'a str) -> Self {
        Self { parser: Parser::new(input, source_name) }
    }
    pub fn run(mut self) -> String {
        self.parser.parse_lolcode();
        self.parser.into_html()
    }
}
