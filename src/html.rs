use crate::dom;
use std::collections::HashMap;

pub struct Parser {
    pub pos: usize,    // posは現在のinputの位置を保存する
    pub input: String, // parseする文字列？
}

impl Parser {
    // rootのドキュメントを返す
    pub fn parse(source: String) -> dom::Node {
        let mut nodes: Vec<dom::Node> = Parser {
            pos: 0,
            input: source,
        }
        .parse_nodes();

        // 要素が一つしかないならswap_removeで最初の要素を消してる
        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            dom::elem("html".to_string(), HashMap::new(), nodes)
        }
    }

    // selfを受け取ってNodeの配列を返す
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        // 配列の初期化
        let mut nodes: Vec<dom::Node> = Vec::new();
        // 文字列の読み取りが終わる or 閉じるタグから文字列が始まってたらループから抜ける
        // それまではparse_nodeしたものを配列に入れ続ける
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            // parse_nodeの中でまだelementがあるならparse_nodesがまた呼ばれるので再帰的にchildrenに入れられる
            nodes.push(self.parse_node());
        }
        return nodes;
    }

    // nodeをparseする
    fn parse_node(&mut self) -> dom::Node {
        // 次の文字を見て'<'ならparse_element()でそれ以外ならparse_text()
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    // elementをparseする
    fn parse_element(&mut self) -> dom::Node {
        // '<'でないならエラー
        assert!(self.consume_char() == '<');
        // '<' から始まっているので 空白か'/>'がきたら終わってタグ名が取れる
        let tag_name: String = self.parse_tag_name();

        let attrs: HashMap<String, String> = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // 中身、Nodeの中のchildrenにNodeが入るのはこれ
        let children: Vec<dom::Node> = self.parse_nodes();

        // '</'がないなら閉じてないのならエラー
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::elem(tag_name, attrs, children);
    }

    // タグの名前をとってくる
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c: char| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    // attributesを見る
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes: HashMap<String, String> = HashMap::new();
        loop {
            self.consume_whitespace();
            // タグが終了するまで見る
            if self.next_char() == '>' {
                break;
            }
            // parse_attr()で次のattributesまで飛んでる
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    // class = "className"とかとってきてくれる
    fn parse_attr(&mut self) -> (String, String) {
        println!("parse_attr");
        // '='になるまで見るのでtag_nameを見れる
        let name: String = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        // valueはどこで終わる？
        let value: String = self.parse_attr_value();
        return (name, value);
    }

    // class = "className"の""の中をとってきてくれる
    fn parse_attr_value(&mut self) -> String {
        let open_quote: char = self.consume_char();
        // ' とか " でないとエラー
        assert!(open_quote == '"' || open_quote == '\'');
        // ' or " まで消費する
        let value: String = self.consume_while(|c: char| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    // 次の文字の値を見る
    fn next_char(&self) -> char {
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

    fn consume_char(&mut self) -> char {
        // 文字列をスライスにしている
        let mut iter: std::str::CharIndices = self.input[self.pos..].char_indices();
        // 現在の文字を1つ後ろにずらして保存
        let (_, cur_char) = iter.next().unwrap();
        // 次の文字の位置を取得している(iterはすでに見られた値は入っていないので常に1になる？)
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        // 現在の位置に次の文字の位置を足している（だいたい１）
        self.pos += next_pos;
        // 現在の見ている文字を返している
        return cur_char;
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        // ジェネリクスの型を制限するためのwhere
        F: Fn(char) -> bool,
    {
        // 空のString型を作る
        let mut result: String = String::new();
        // 見る文字列がなくなる and 関数の条件を満たさなくなる　まではループ
        while !self.eof() && test(self.next_char()) {
            // 見た文字列をresultに入れてる
            result.push(self.consume_char());
        }
        return result;
    }
    // 空白が出た時点でループを中断する
    fn consume_whitespace(&mut self) {
        self.consume_while(|c: char| c.is_whitespace());
    }

    // 次の文字列が'<'でないところまでの文字列を返す
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c: char| c != '<'))
    }
}
