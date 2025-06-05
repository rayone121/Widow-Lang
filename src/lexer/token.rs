use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\f]+")] // Skip whitespace but not newlines
pub enum Token {
    // Keywords - order matters for longest match
    #[token("async")]
    Async,
    #[token("await")]
    Await,
    #[token("break")]
    Break,
    #[token("case")]
    Case,
    #[token("const")]
    Const,
    #[token("continue")]
    Continue,
    #[token("default")]
    Default,
    #[token("elif")]
    Elif,
    #[token("else")]
    Else,
    #[token("enumerate")]
    Enumerate,
    #[token("false")]
    False,
    #[token("for")]
    For,
    #[token("from")]
    From,
    #[token("func")]
    Func,
    #[token("if")]
    If,
    #[token("impl")]
    Impl,
    #[token("import")]
    Import,
    #[token("in")]
    In,
    #[token("match")]
    Match,
    #[token("module")]
    Module,
    #[token("nil")]
    Nil,
    #[token("ret")]
    Return,
    #[token("self")]
    SelfKeyword,
    #[token("spawn")]
    Spawn,
    #[token("step")]
    Step,
    #[token("struct")]
    Struct,
    #[token("switch")]
    Switch,
    #[token("then")]
    Then,
    #[token("trait")]
    Trait,
    #[token("true")]
    True,
    #[token("while")]
    While,
    #[token("with")]
    With,
    #[token("as")]
    As,

    // Type keywords
    #[token("bool")]
    BoolType,
    #[token("char")]
    CharType,
    #[token("f32")]
    F32,
    #[token("f64")]
    F64,
    #[token("i16")]
    I16,
    #[token("i32")]
    I32,
    #[token("i64")]
    I64,
    #[token("i8")]
    I8,
    #[token("map")]
    MapType,
    #[token("set")]
    SetType,
    #[token("String")]
    StringType,
    #[token("u16")]
    U16,
    #[token("u32")]
    U32,
    #[token("u64")]
    U64,
    #[token("u8")]
    U8,

    // Multi-character operators (must come before single character ones)
    #[token("**")]
    Power,
    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    #[token("*=")]
    MultiplyAssign,
    #[token("/=")]
    DivideAssign,
    #[token("%=")]
    ModuloAssign,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,
    #[token("..=")]
    RangeInclusive,
    #[token("..")]
    Range,
    #[token("?.")]
    SafeAccess,
    #[token("??")]
    NullCoalescing,
    #[token("->")]
    Arrow,
    #[token("::")]
    DoubleColon,
    #[token("...")]
    Ellipsis,

    // Single character operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("=")]
    Assign,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("!")]
    Not,
    #[token("&")]
    BitwiseAnd,
    #[token("|")]
    BitwiseOr,
    #[token("^")]
    BitwiseXor,
    #[token("~")]
    BitwiseNot,

    // Punctuation
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("?")]
    Question,
    #[token("@")]
    At,
    #[token("#")]
    Hash,
    #[token("$")]
    Dollar,

    // String literals - raw strings must come before regular strings
    #[regex(r#"r"([^"]*)""#, |lex| lex.slice()[2..lex.slice().len()-1].to_string())]
    RawString(String),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let slice = lex.slice();
        slice[1..slice.len()-1].to_string()
    })]
    String(String),

    #[regex(r"`([^`\\]|\\.)*`", |lex| {
        let slice = lex.slice();
        slice[1..slice.len()-1].to_string()
    })]
    TemplateString(String),

    // Character literals
    #[regex(r"'([^'\\]|\\.)'", |lex| {
        let slice = lex.slice();
        slice.chars().nth(1).unwrap()
    })]
    Character(char),

    // Numeric literals - floats must come before integers
    #[regex(r"\d+\.\d+([eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[regex(r"\d+", |lex| lex.slice().parse::<i64>().unwrap())]
    Integer(i64),

    // Identifiers (must come after keywords)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Comments - order matters for proper matching
    #[regex(r"(?s)/\*\*([^*]|\*+[^*/])*\*+/", |lex| lex.slice().to_string())]
    DocComment(String),

    #[regex(r"(?s)/\*([^*]|\*+[^*/])*\*+/", |lex| lex.slice().to_string())]
    BlockComment(String),

    #[regex(r"//[^\n\r]*", |lex| lex.slice().to_string())]
    LineComment(String),

    // Newlines (significant for parsing)
    #[token("\n")]
    Newline,

    // Error handling
    Error,
}

impl Token {
    /// Returns true if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(self,
            Token::Async | Token::Await | Token::Break | Token::Case |
            Token::Const | Token::Continue | Token::Default | Token::Elif |
            Token::Else | Token::Enumerate | Token::False | Token::For |
            Token::From | Token::Func | Token::If | Token::Impl |
            Token::Import | Token::In | Token::Match | Token::Module |
            Token::Nil | Token::Return | Token::SelfKeyword | Token::Spawn |
            Token::Step | Token::Struct | Token::Switch | Token::Then |
            Token::Trait | Token::True | Token::While | Token::With | Token::As
        )
    }

    /// Returns true if this token is a literal value
    pub fn is_literal(&self) -> bool {
        matches!(self,
            Token::Integer(_) | Token::Float(_) | Token::String(_) |
            Token::RawString(_) | Token::TemplateString(_) | Token::Character(_) |
            Token::True | Token::False | Token::Nil
        )
    }

    /// Returns true if this token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(self,
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide |
            Token::Modulo | Token::Power | Token::Assign | Token::PlusAssign |
            Token::MinusAssign | Token::MultiplyAssign | Token::DivideAssign |
            Token::ModuloAssign | Token::Equal | Token::NotEqual |
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual |
            Token::And | Token::Or | Token::Not | Token::BitwiseAnd |
            Token::BitwiseOr | Token::BitwiseXor | Token::BitwiseNot |
            Token::LeftShift | Token::RightShift | Token::Range |
            Token::RangeInclusive | Token::SafeAccess | Token::NullCoalescing |
            Token::Arrow
        )
    }

    /// Returns true if this token is a type annotation
    pub fn is_type(&self) -> bool {
        matches!(self,
            Token::I8 | Token::I16 | Token::I32 | Token::I64 |
            Token::U8 | Token::U16 | Token::U32 | Token::U64 |
            Token::F32 | Token::F64 | Token::StringType | Token::BoolType |
            Token::CharType | Token::MapType | Token::SetType
        )
    }

    /// Returns true if this token is a comment
    pub fn is_comment(&self) -> bool {
        matches!(self,
            Token::LineComment(_) | Token::BlockComment(_) |
            Token::DocComment(_)
        )
    }

    /// Returns true if this token can start an expression
    pub fn can_start_expression(&self) -> bool {
        matches!(self,
            Token::Identifier(_) | Token::Integer(_) | Token::Float(_) |
            Token::String(_) | Token::RawString(_) | Token::TemplateString(_) |
            Token::Character(_) | Token::True | Token::False | Token::Nil |
            Token::LeftParen | Token::LeftBracket | Token::LeftBrace |
            Token::Not | Token::Minus | Token::Plus | Token::BitwiseNot |
            Token::If | Token::Match | Token::Switch | Token::Func |
            Token::SelfKeyword
        )
    }

    /// Returns the precedence of this operator token (higher number = higher precedence)
    pub fn precedence(&self) -> Option<u8> {
        match self {
            Token::Or => Some(1),
            Token::And => Some(2),
            Token::Equal | Token::NotEqual => Some(3),
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual => Some(4),
            Token::BitwiseOr => Some(5),
            Token::BitwiseXor => Some(6),
            Token::BitwiseAnd => Some(7),
            Token::LeftShift | Token::RightShift => Some(8),
            Token::Plus | Token::Minus => Some(9),
            Token::Multiply | Token::Divide | Token::Modulo => Some(10),
            Token::Power => Some(11),
            Token::Not | Token::BitwiseNot => Some(12), // Unary operators
            Token::Dot | Token::SafeAccess => Some(13), // Member access
            _ => None,
        }
    }

    /// Returns true if this operator is right-associative
    pub fn is_right_associative(&self) -> bool {
        matches!(self, Token::Power | Token::Assign | Token::PlusAssign |
                 Token::MinusAssign | Token::MultiplyAssign | Token::DivideAssign |
                 Token::ModuloAssign)
    }

    /// Returns the string representation of the token for display
    pub fn as_str(&self) -> &'static str {
        match self {
            Token::Async => "async",
            Token::Await => "await",
            Token::Break => "break",
            Token::Case => "case",
            Token::Const => "const",
            Token::Continue => "continue",
            Token::Default => "default",
            Token::Elif => "elif",
            Token::Else => "else",
            Token::Enumerate => "enumerate",
            Token::False => "false",
            Token::For => "for",
            Token::From => "from",
            Token::Func => "func",
            Token::If => "if",
            Token::Impl => "impl",
            Token::Import => "import",
            Token::In => "in",
            Token::Match => "match",
            Token::Module => "module",
            Token::Nil => "nil",
            Token::Return => "ret",
            Token::SelfKeyword => "self",
            Token::Spawn => "spawn",
            Token::Step => "step",
            Token::Struct => "struct",
            Token::Switch => "switch",
            Token::Then => "then",
            Token::Trait => "trait",
            Token::True => "true",
            Token::While => "while",
            Token::With => "with",
            Token::As => "as",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Multiply => "*",
            Token::Divide => "/",
            Token::Modulo => "%",
            Token::Power => "**",
            Token::Assign => "=",
            Token::PlusAssign => "+=",
            Token::MinusAssign => "-=",
            Token::MultiplyAssign => "*=",
            Token::DivideAssign => "/=",
            Token::ModuloAssign => "%=",
            Token::Equal => "==",
            Token::NotEqual => "!=",
            Token::Less => "<",
            Token::LessEqual => "<=",
            Token::Greater => ">",
            Token::GreaterEqual => ">=",
            Token::And => "&&",
            Token::Or => "||",
            Token::Not => "!",
            Token::BitwiseAnd => "&",
            Token::BitwiseOr => "|",
            Token::BitwiseXor => "^",
            Token::BitwiseNot => "~",
            Token::LeftShift => "<<",
            Token::RightShift => ">>",
            Token::Range => "..",
            Token::RangeInclusive => "..=",
            Token::SafeAccess => "?.",
            Token::NullCoalescing => "??",
            Token::Arrow => "->",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftBracket => "[",
            Token::RightBracket => "]",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::Colon => ":",
            Token::DoubleColon => "::",
            Token::Dot => ".",
            Token::Ellipsis => "...",
            Token::Question => "?",
            Token::At => "@",
            Token::Hash => "#",
            Token::Dollar => "$",
            Token::Newline => "\\n",
            Token::Error => "ERROR",
            _ => "COMPLEX_TOKEN",
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "identifier '{}'", s),
            Token::Integer(n) => write!(f, "integer {}", n),
            Token::Float(n) => write!(f, "float {}", n),
            Token::String(s) => write!(f, "string \"{}\"", s),
            Token::RawString(s) => write!(f, "raw string r\"{}\"", s),
            Token::TemplateString(s) => write!(f, "template string `{}`", s),
            Token::Character(c) => write!(f, "character '{}'", c),
            Token::LineComment(s) => write!(f, "line comment {}", s),
            Token::BlockComment(s) => write!(f, "block comment {}", s),
            Token::DocComment(s) => write!(f, "doc comment {}", s),
            _ => write!(f, "{}", self.as_str()),
        }
    }
}