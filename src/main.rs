use std::println;

use crate::html::Parser;

pub mod css;
pub mod dom;
pub mod html;
pub mod style;
fn main() {
    // Node 動作確認
    // let sample_node: dom::Node = dom::Node {
    //     children: vec![],
    //     node_type: dom::NodeType::Text("<h1>sample-h1</h1>".to_string()),
    // };
    // println!("{:?}", sample_node.node_type);

    println!("----------------------------------------------------------------------");
    // HTML動作確認
    let dom: dom::Node = Parser::parse(
        "<h1 class='sample' style='font-bold'>コンテンツ<p class='sample2'>Aです</p><p>Bです</p></h1>".to_string(),
    );
    // println!("{:?}", dom);

    println!("\n");
    let css: css::Stylesheet = crate::css::parse(
        "h1{ margin: auto; backgroundcolor: red; } .sample{ padding: 10px;} .sample2{ text-align: center; }".to_string(),
    );
    // CSS動作確認
    // println!("{:?}", css);

    let styletree: style::StyledNode = crate::style::style_tree(&dom, &css);
    println!("{:?}", styletree);
}
