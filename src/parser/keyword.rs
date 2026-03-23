use std::{collections::HashMap, sync::LazyLock};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Keyword {
    // Declarations
    Let,     // let
    Mut,     // mut
    Set,     // set!
    Def,     // def
    Closure, // fn

    // Control flows
    If,
    Else,
    Elif,
    Block,
    Or,
    And,
    Not,
}

static KEYWORDS: LazyLock<HashMap<&'static str, Keyword>> = LazyLock::new(|| {
    HashMap::from([
        ("let", Keyword::Let),
        ("mut", Keyword::Mut),
        ("set!", Keyword::Set),
        ("def", Keyword::Def),
        ("fn", Keyword::Closure),
        ("if", Keyword::If),
        (":else", Keyword::Else),
        (":elif", Keyword::Elif),
        ("block", Keyword::Block),
        ("or", Keyword::Or),
        ("and", Keyword::And),
        ("not", Keyword::Not),
    ])
});

pub fn is_keyword(symbol: &str) -> bool {
    KEYWORDS.contains_key(symbol)
}

pub fn parse_keyword(symbol: &str) -> Keyword {
    *KEYWORDS.get(symbol).unwrap()
}
