use logos::Lexer;

use super::Token;

pub struct Include;

impl Include {
    pub fn extract(lexer: &mut Lexer<Token>) -> String {
        // TODO: エラーハンドリング
        // いったんノーガード
        let keyword = lexer.next().unwrap().unwrap();

        // #include のあとは文字列を期待
        match keyword {
            Token::String => lexer.slice().to_string(),
            Token::Include => todo!(),
            Token::Camera => todo!(),
            Token::BracketLeft => todo!(),
            Token::BracketRight => todo!(),
        }
    }
}
