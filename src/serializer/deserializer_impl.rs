use logos::Logos;

use crate::serializer::detail::{self, Token};

use super::ISceneBuilder;

pub struct Deserializer;

impl Deserializer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn deserialize<R, T>(&self, mut reader: R, scene_builder: &mut T) -> Result<(), ()>
    where
        R: std::io::Read,
        T: ISceneBuilder,
    {
        // pov ファイルを文字列として読み込む
        let mut str = String::new();
        reader.read_to_string(&mut str).unwrap();

        // 字句解析
        let mut lexer = Token::lexer(&str);

        // 構文解析の起点となるキーワードだけ分岐で判定
        // その詳細な解析は各種 detail 実装に逃す
        while let Some(keyword) = lexer.next() {
            let keyword = keyword.unwrap();

            match keyword {
                Token::Include => {
                    // インクルードファイルのパース
                    let include = detail::Include::extract(&mut lexer);
                    scene_builder.add_include_path(include);
                }
                Token::String => {
                    // ダブルクォーテーションで囲われた文字列がいきなり現れることはないはず
                    todo!()
                }
                Token::Camera => {
                    // TODO
                }
                Token::BracketLeft => todo!(),
                Token::BracketRight => todo!(),
            }
        }

        Ok(())
    }
}
