//! LLVM IR lexer using logos

use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace
#[logos(skip r";[^\n]*")] // Skip comments
pub enum Token {
    // Keywords
    #[token("define")]
    Define,

    #[token("declare")]
    Declare,

    #[token("ret")]
    Ret,

    #[token("br")]
    Br,

    #[token("switch")]
    Switch,

    #[token("call")]
    Call,

    #[token("add")]
    Add,

    #[token("sub")]
    Sub,

    #[token("mul")]
    Mul,

    #[token("udiv")]
    UDiv,

    #[token("sdiv")]
    SDiv,

    #[token("urem")]
    URem,

    #[token("srem")]
    SRem,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("xor")]
    Xor,

    #[token("shl")]
    Shl,

    #[token("lshr")]
    LShr,

    #[token("ashr")]
    AShr,

    #[token("icmp")]
    ICmp,

    #[token("load")]
    Load,

    #[token("store")]
    Store,

    #[token("alloca")]
    Alloca,

    #[token("getelementptr")]
    GetElementPtr,

    #[token("phi")]
    Phi,

    #[token("label")]
    Label,

    #[token("to")]
    To,

    // Comparison predicates
    #[token("eq")]
    Eq,

    #[token("ne")]
    Ne,

    #[token("slt")]
    Slt,

    #[token("sle")]
    Sle,

    #[token("sgt")]
    Sgt,

    #[token("sge")]
    Sge,

    #[token("ult")]
    Ult,

    #[token("ule")]
    Ule,

    #[token("ugt")]
    Ugt,

    #[token("uge")]
    Uge,

    // Types
    #[token("void")]
    Void,

    #[regex(r"i[0-9]+", |lex| lex.slice()[1..].parse().ok())]
    IntType(u32),

    #[token("ptr")]
    Ptr,

    // Literals
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),

    #[token("true")]
    True,

    #[token("false")]
    False,

    // Identifiers
    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    GlobalIdent(String),

    #[regex(r"%[a-zA-Z_][a-zA-Z0-9_.]*", |lex| lex.slice().to_string())]
    LocalIdent(String),

    #[regex(r"%[0-9]+", |lex| lex.slice().to_string())]
    NumericIdent(String),

    // Punctuation
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("<")]
    LAngle,

    #[token(">")]
    RAngle,

    #[token(",")]
    Comma,

    #[token("=")]
    Equals,

    #[token("*")]
    Star,

    #[token(":")]
    Colon,

    #[token("...")]
    Ellipsis,

    // String literals
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().to_string())]
    String(String),
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    token_stream: logos::Lexer<'input, Token>,
    position: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Token::lexer(input),
            position: 0,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.token_stream.next()?;
        let span = self.token_stream.span();

        match token {
            Ok(tok) => {
                let result = Ok((span.start, tok, span.end));
                self.position = span.end;
                Some(result)
            }
            Err(_) => Some(Err(format!(
                "Unexpected character at position {}",
                span.start
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic() {
        let input = "define i32 @main() { ret i32 0 }";
        let mut lexer = Lexer::new(input);

        assert!(matches!(lexer.next(), Some(Ok((_, Token::Define, _)))));
        assert!(matches!(lexer.next(), Some(Ok((_, Token::IntType(32), _)))));
        assert!(matches!(
            lexer.next(),
            Some(Ok((_, Token::GlobalIdent(_), _)))
        ));
    }

    #[test]
    fn test_lexer_identifiers() {
        let input = "@global %local %123";
        let mut lexer = Lexer::new(input);

        if let Some(Ok((_, Token::GlobalIdent(s), _))) = lexer.next() {
            assert_eq!(s, "@global");
        }

        if let Some(Ok((_, Token::LocalIdent(s), _))) = lexer.next() {
            assert_eq!(s, "%local");
        }

        if let Some(Ok((_, Token::NumericIdent(s), _))) = lexer.next() {
            assert_eq!(s, "%123");
        }
    }
}
