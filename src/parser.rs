pub struct Parser {
    pos: usize,    // posは現在のinputの位置を保存する
    input: String, // parseする文字列？
}

impl Parser {
    // 次の文字列があるかどうか見る
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
        let mut iter = self.input[self.pos..].char_indices();
        // 現在の文字を1つ後ろにずらして保存
        let (_, cur_char) = iter.next().unwrap();
        // 次の文字の位置を取得している
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        //現在の位置に次の文字の位置を足している
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
        let mut result = String::new();
        // 見る文字列がなくなるまでループを回し続ける
        while !self.eof() && test(self.next_char()) {
            // 見た文字列をresultに入れてる
            result.push(self.consume_char());
        }
        return result;
    }
    // 空白を無視して消費する
    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    // タグに名前をつける
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    // Parse a single node.
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
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
}
