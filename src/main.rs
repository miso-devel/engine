use crate::html::Parser;

pub mod css;
pub mod dom;
pub mod html;

fn main() {
    // Node 動作確認
    let sample_node = dom::Node {
        children: vec![],
        node_type: dom::NodeType::Text("<h1>sample-h1</h1>".to_string()),
    };
    println!("{:?}", sample_node.node_type);

    println!("----------------------------------------------------------------------");
    // HTML動作確認
    println!(
        "{:?}",
        Parser::parse(
            "<h1 class='sample' style='font-bold'>あああ<p>pタグ</p>h1タグ</h1>".to_string()
        )
    );

    println!("\n");
    // CSS動作確認
    println!(
        "{:?}",
        crate::css::parse("h1.a.b#c#d{ margin: auto; backgroundcolor: red; }".to_string())
    )
}
