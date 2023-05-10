#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    // styleを当てる対象
    pub selectors: Vec<Selector>,
    // style自体
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

// style の margin(name): auto(value); の部分
#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    // これはタプル
    Length(f32, Unit),
    ColorValue(Color),
}

// 文字とかの高さとかの単位？
#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Px,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Copy for Color {}

pub type Specificity = (usize, usize, usize);

impl Selector {
    // それぞれの長さとか数をとってる
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        let a: usize = simple.id.iter().count();
        let b: usize = simple.class.len();
        let c: usize = simple.tag_name.iter().count();
        (a, b, c)
    }
}

impl Value {
    //　ValueがLengthならfを返すしそれ以外なら0を返す
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0,
        }
    }
}

//　最終的に呼び出されるcssをparse関数
pub fn parse(source: String) -> Stylesheet {
    let mut parser: Parser = Parser {
        pos: 0,
        input: source,
    };
    Stylesheet {
        rules: parser.parse_rules(),
    }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    // Vex<rule>を返しているのでstyleは実際ここがparseしている
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules: Vec<Rule> = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            rules.push(self.parse_rule());
        }
        rules
    }

    //　selectors(margin) : declarations(auto);　みたいなのをここで作ってる
    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    //　selectors(h1.style)みたいな感じがあるからVec
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors: Vec<Selector> = Vec::new();
        loop {
            // Selector::Simple(self.parse_simple_selector())を
            println!(
                "parse_selectors:{:?}",
                Selector::Simple(self.parse_simple_selector())
            );
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            // 次の文字が,なら次のselectorに移る、{なら終わる
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }
        // ちょっと意味わかってない、styleを適用する順番を決めてたりする
        selectors.sort_by(|a: &Selector, b: &Selector| b.specificity().cmp(&a.specificity()));
        selectors
    }

    /// selectorを決定している
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        // selectorの初期化
        let mut selector: SimpleSelector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        // 全て読み終わるまで実行
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    // universal selector
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }
        selector
    }

    /// styleのvecを返す
    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.consume_char(), '{');
        let mut declarations: Vec<Declaration> = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    /// :で区切っている。なんとなくわかる。
    fn parse_declaration(&mut self) -> Declaration {
        let property_name: String = self.parse_identifier();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ':');
        self.consume_whitespace();
        let value: Value = self.parse_value();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ';');

        Declaration {
            name: property_name,
            value: value,
        }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float(&mut self) -> f32 {
        let s: String = self.consume_while(|c: char| match c {
            '0'..='9' | '.' => true,
            _ => false,
        });
        s.parse().unwrap()
    }

    // pxならそう認識する。最終的にUnitを返してる
    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit"),
        }
    }

    // これも色をなんとかしている
    fn parse_color(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255,
        })
    }

    /// 色の決定のためになんかしてる
    fn parse_hex_pair(&mut self) -> u8 {
        let s: &str = &self.input[self.pos..self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }

    // validateしながら消費
    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    // 空白を無視
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // まあ、htmlの方と同じ
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result: String = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    //　読んだ文字を返して、次にposを進める
    fn consume_char(&mut self) -> char {
        let mut iter: std::str::CharIndices = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    // 次に進むことなく次の文字を読む
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// 全て読み終わったらtrueを返す
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

// margin: auto;みたいなのでmarginとautoをとってくるためのvaidator
// .style #style　とかのstyleの部分だけ取ってくる用途でも使ってる
fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true, // TODO: Include U+00A0 and higher.
        _ => false,
    }
}
