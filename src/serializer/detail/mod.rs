mod camera;
mod include;

pub use camera::Camera;
pub use include::Include;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[token("#include")]
    Include,

    #[regex("\"[a-zA-Z.]+\"")]
    String,

    #[token("camera")]
    Camera,

    #[token("{")]
    BracketLeft,

    #[token("}")]
    BracketRight,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn include() {
        // #include"aaa" のパース
        let mut lex = Token::lexer(include_str!("tests/include.pov"));

        assert_eq!(lex.next(), Some(Ok(Token::Include)));
        assert_eq!(lex.next(), Some(Ok(Token::String)));
    }

    #[test]
    fn include_space() {
        // #include と "aaa" の間にスペースがあるケース
        let mut lex = Token::lexer(include_str!("tests/include_space.pov"));

        assert_eq!(lex.next(), Some(Ok(Token::Include)));
        assert_eq!(lex.next(), Some(Ok(Token::String)));
    }

    #[test]
    fn include_with_keyword() {
        // インクルードするファイル名にキーワードが入ってるケース
        let mut lex = Token::lexer(include_str!("tests/include_keyword.pov"));

        assert_eq!(lex.next(), Some(Ok(Token::Include)));
        assert_eq!(lex.next(), Some(Ok(Token::String)));
    }

    #[test]
    fn camera() {
        // camera 設定のパース
        let mut lex = Token::lexer(include_str!("tests/camera.pov"));

        assert_eq!(lex.next(), Some(Ok(Token::Camera)));
        assert_eq!(lex.next(), Some(Ok(Token::BracketLeft)));
        assert_eq!(lex.next(), Some(Ok(Token::BracketRight)));
    }
}
