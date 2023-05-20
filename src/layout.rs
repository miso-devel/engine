use crate::style::{self, StyledNode};
use std::default::Default;

// layout.rsは要素の位置を計算するためのファイル

#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Default, Debug)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

// blockとかinlineとか（今は実装されてないけどflex）とか並び方を決めるもの
pub enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    InlineNode(&'a StyledNode<'a>),
    AnonymousBlock,
}

// これが全体を表してそう
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
    pub fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
            BoxType::BlockNode(_) => {
                // もしBlockなら空のオブジェクトを返してそうでないなら匿名のBlockを返している？？
                // これは良くやってることがわかってない
                match self.children.last() {
                    Some(&LayoutBox {
                        box_type: BoxType::AnonymousBlock,
                        ..
                    }) => {}
                    _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type: box_type,
            dimensions: Default::default(), // initially set all fields to 0.0
            children: Vec::new(),
        }
    }
}

// layout_treeを作る
pub fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    // 親のbox作る
    let mut root: LayoutBox = LayoutBox::new(match style_node.display() {
        style::Display::Block => BoxType::BlockNode(style_node),
        style::Display::Inline => BoxType::InlineNode(style_node),
        style::Display::None => panic!("Root node has display: none."),
    });

    // 子孫のboxを作る
    // forで回してる。どっかで再帰的に読んでそう
    for child in &style_node.children {
        match child.display() {
            style::Display::Block => root.children.push(build_layout_tree(child)),
            style::Display::Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(child)),
            style::Display::None => {} // Skip nodes with `display: none;`
        }
    }
    return root;
}
