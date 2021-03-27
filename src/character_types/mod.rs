/// Macro characters are divided into two kinds, terminating and non-terminating, depending on whether or not they terminate a token
pub enum MacroCharKind {
    Terminating,
    NonTerminating,
}

pub enum DotExt {
    DecimalPoint,
}

pub enum AlphabeticExt {
    PlusSign,
    MinusSign,
    Dot { ext: Option<DotExt> },
    RatioMarker,
}

pub enum AlphadigitExt {
    ExponentMarker,
}

pub enum ConsTrait {
    Alphabetic { ext: Option<AlphabeticExt> },
    Digit,
    Alphadigit { ext: Option<AlphadigitExt> },
    PackageMarker,
    Invalid,
}

/// One of several classifications, enumerated in Figure 2-6, that are used for dispatch during parsing by the Lisp reader
pub enum SyntaxType {
    /// Constituent characters are used in tokens
    Constituent { r#traits: ConsTrait },
    MacroCharacter { kind: MacroCharKind },
    SingleEscape,
    MultipleEscape,
    Whitespace,
    Invalid,
}



pub trait Character {
    fn syntax_type(&self) -> SyntaxType;
}

impl Character for char {
    fn syntax_type(&self) -> SyntaxType {
        match self {
            '\u{8}' => SyntaxType::Constituent { r#traits: ConsTrait::Invalid },
            '!' | '$' | '%' | '&' | '*' | '+' | '-' | '<' | '=' | '>' | '?' | '@' | '[' | ']' | '^' | '_' | '{' | '}' | '~' => SyntaxType::Constituent { r#traits: ConsTrait::Alphabetic { ext: None }},
            '.' => SyntaxType::Constituent { r#traits: ConsTrait::Alphabetic { ext: Some(AlphabeticExt::Dot { ext: Some(DotExt::DecimalPoint) }) } },
            '/' => SyntaxType::Constituent { r#traits: ConsTrait::Alphabetic { ext: Some(AlphabeticExt::RatioMarker) } },
            '0'..= '9' | 'a' ..= 'c' | 'A' ..= 'C' | 'g' ..= 'k' | 'G' ..= 'K' | 'm' ..= 'r' | 'M' ..= 'R' | 't' ..= 'z' | 'T' ..= 'Z' => SyntaxType::Constituent { r#traits: ConsTrait::Alphadigit { ext: None }},
            'd' ..= 'f' | 'D' ..= 'F' | 'l' | 's' | 'L' | 'S' => SyntaxType::Constituent { r#traits: ConsTrait::Alphadigit { ext: Some(AlphadigitExt::ExponentMarker) }},
            ':' => SyntaxType::Constituent { r#traits: ConsTrait::PackageMarker },
            '\u{9}' | '\n' | '\u{C}' | '\u{D}' | '\u{20}' => SyntaxType::Whitespace,
            '"' | '\'' | '(' | ')' | ',' | ';' | '`' => SyntaxType::MacroCharacter {
                kind: MacroCharKind::Terminating,
            },
            '#' => SyntaxType::MacroCharacter {
                kind: MacroCharKind::NonTerminating,
            },
            '\\' => SyntaxType::SingleEscape,
            '|' => SyntaxType::MultipleEscape,
            _ => SyntaxType::Invalid,
        }
    }
}
