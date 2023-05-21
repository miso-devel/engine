use crate::{
    css,
    style::{self, StyledNode},
};
use std::default::Default;

// layout.rsは要素の位置を計算するためのファイル

#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl Dimensions {
    // The area covered by the content area plus its padding.
    fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }
    // The area covered by the content area plus padding and borders.
    fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }
    // The area covered by the content area plus padding, borders, and margin.
    fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

// blockとかinlineとか（今は実装されてないけどflex）とか並び方を決めるもの
#[derive(Clone, Copy, Debug)]
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
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(), // initially set all fields to 0.0
            children: Vec::new(),
        }
    }

    fn get_style_node(&self) -> &'a StyledNode<'a> {
        match self.box_type {
            BoxType::BlockNode(node) | BoxType::InlineNode(node) => node,
            BoxType::AnonymousBlock => panic!("Anonymous block box has no style node"),
        }
    }
}

impl<'a> LayoutBox<'a> {
    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(containing_block),
            BoxType::InlineNode(_) => {}  // TODO
            BoxType::AnonymousBlock => {} // TODO
        }
    }

    // 幅を計算するときはツリーを上から下へ走査し、親の幅がわかってから子を配置し、高さを計算するときは下から上へ走査し、親の高さは子の高さの後に計算する必要がある
    fn layout_block(&mut self, containing_block: Dimensions) {
        // 子要素の幅を計算する
        self.calculate_block_width(containing_block);

        // Determine where the box is located within its container.
        self.calculate_block_position(containing_block);

        // Recursively lay out the children of this box.
        self.layout_block_children();

        // Parent height can depend on child height, so `calculate_height`
        // must be called *after* the children are laid out.
        self.calculate_block_height();
    }

    // Block要素の幅を計算する
    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style: &StyledNode = self.get_style_node();

        // 初期値はauto
        let auto: css::Value = css::Value::Keyword("auto".to_string());
        let mut width: css::Value = style.value("width").unwrap_or(auto.clone());

        // marginとか色々初期値は0
        let zero: css::Value = css::Value::Length(0.0, css::Unit::Px);

        // lookupは引数があるかどうか順に試している？？
        let mut margin_left: css::Value = style.lookup("margin-left", "margin", &zero);
        let mut margin_right: css::Value = style.lookup("margin-right", "margin", &zero);

        let border_left: css::Value = style.lookup("border-left-width", "border-width", &zero);
        let border_right: css::Value = style.lookup("border-right-width", "border-width", &zero);

        let padding_left: css::Value = style.lookup("padding-left", "padding", &zero);
        let padding_right: css::Value = style.lookup("padding-right", "padding", &zero);
    }

    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let d: &mut Dimensions = &mut self.dimensions;

        // margin, border, and padding have initial value 0.
        let zero: css::Value = css::Value::Length(0.0, css::Unit::Px);

        // If margin-top or margin-bottom is `auto`, the used value is zero.
        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style
            .lookup("border-top-width", "border-width", &zero)
            .to_px();
        d.border.bottom = style
            .lookup("border-bottom-width", "border-width", &zero)
            .to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;

        // Position the box below all the previous boxes in the container.
        d.content.y = containing_block.content.height
            + containing_block.content.y
            + d.margin.top
            + d.border.top
            + d.padding.top;
    }

    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            // Track the height so each child is laid out below the previous content.
            d.content.height = d.content.height + child.dimensions.margin_box().height;
        }
    }

    fn calculate_block_height(&mut self) {
        // If the height is set to an explicit length, use that exact length.
        // Otherwise, just keep the value set by `layout_block_children`.
        if let Some(css::Value::Length(h, css::Unit::Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }

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
