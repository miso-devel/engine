use crate::dom;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Parser {
    pub pos: usize,    // posは現在のinputの位置を保存する
    pub input: String, // parseする文字列？
}

impl Parser {
    // 次の文字の値を見る
    pub fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // inputがsで始まっているのかどうか見る
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    //　全ての文字列を見終わったかどうか見る
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn consume_char(&mut self) -> char {
        // 文字列をスライスにしている
        let mut iter = self.input[self.pos..].char_indices();
        // 現在の文字を1つ後ろにずらして保存
        let (_, cur_char) = iter.next().unwrap();
        // 次の文字の位置を取得している(iterはすでに見られた値は入っていないので常に1になる？)
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        // 現在の位置に次の文字の位置を足している（だいたい１）
        self.pos += next_pos;
        // 現在の見ている文字を返している
        return cur_char;
    }

    pub fn consume_while<F>(&mut self, test: F) -> String
    where
        // ジェネリクスの型を制限するためのwhere
        F: Fn(char) -> bool,
    {
        // 空のString型を作る
        let mut result = String::new();
        // println!(
        //     "result:{:?}   eof:{:?}  test:{:?} ",
        //     result,
        //     !self.eof(),
        //     test(self.next_char())
        // );
        // 見る文字列がなくなる and 関数の条件を満たさなくなる　まではループ
        while !self.eof() && test(self.next_char()) {
            // 見た文字列をresultに入れてる
            // println!(
            //     "result:{:?}   eof:{:?}  test:{:?} ",
            //     result,
            //     !self.eof(),
            //     test(self.next_char())
            // );
            result.push(self.consume_char());
        }
        return result;
    }
    // 空白が出た時点でループを中断する
    pub fn consume_whitespace(&mut self) {
        self.consume_while(|c: char| !c.is_whitespace());
    }

    // タグに名前をつける??
    // なんか違う気がするな,<h2>h2element</h2>を入れても何も返ってこない（h2が返ってくるイメージだった）
    pub fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c: char| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    // nodeをparseする
    fn parse_node(&mut self) -> dom::Node {
        // 次の文字を見て'<'ならparse_element()でそれ以外ならparse_text()
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    // Parse a sequence of sibling nodes.
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }

    // Parse a text node.
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    // Parse a single element, including its open tag, contents, and closing tag.
    fn parse_element(&mut self) -> dom::Node {
        // Opening tag.
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents.
        let children = self.parse_nodes();

        // Closing tag.
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::elem(tag_name, attrs, children);
    }

    // Parse a single name="value" pair.
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    // Parse a quoted value.
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    // Parse a list of name="value" pairs, separated by whitespace.
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    // Parse an HTML document and return the root element.
    pub fn parse(source: String) -> dom::Node {
        let mut nodes = Parser {
            pos: 0,
            input: source,
        }
        .parse_nodes();

        // If the document contains a root element, just return it. Otherwise, create one.
        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            dom::elem("html".to_string(), HashMap::new(), nodes)
        }
    }
}
