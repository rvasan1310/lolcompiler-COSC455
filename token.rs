#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Keywords / tags (case-insensitive in source; store uppercase)
    HashHai,
    HashKthxbye,
    HashObtW,     // #OBTW
    HashTldr,     // #TLDR
    HashMaek,     // #MAEK
    HashOic,      // #OIC
    HashGimmeh,   // #GIMMEH
    HashMKay,     // #MKAY
    Head,
    Title,
    Paragraf,
    Bold,
    Italics,
    List,
    Item,
    Newline,
    Soundz,
    Vidz,
    HashIHaz,     // #I HAZ
    HashItIz,     // #IT IZ
    HashLemmeSee, // #LEMME SEE

    // Text blobs
    Text(String),

    // End of input
    Eof,
}

#[allow(dead_code)]
impl Token {
    pub fn is_text(&self) -> bool {
        matches!(self, Token::Text(_))
    }
}
