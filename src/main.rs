use crate::html::Parser;

pub mod dom;
pub mod html;

fn main() {
    // Node 動作確認
    let sample_node = dom::Node {
        children: vec![],
        node_type: dom::NodeType::Text("<h1>sample-h1</h1>".to_string()),
    };
    println!("{:?}", sample_node.node_type);

    // HTML動作確認
    let mut sample_html = Parser {
        pos: 0,
        input: "<h2>h2element</h2>".to_string(),
    };
    println!("{:?}", sample_html.parse_tag_name());
    // println!("{:?}", sample_html.consume_char());
    // println!("{:?}", sample_html.consume_char());
    // println!("{:?}", sample_html.consume_char());
    // println!("{:?}", sample_html.consume_whitespace());
}
