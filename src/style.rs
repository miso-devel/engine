use std::collections::HashMap;

use crate::{
    css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value},
    dom::{ElementData, Node},
};

// styleを表す型
type PropertyMap = HashMap<String, Value>;

// Nodeに関連したstyleとその子要素を表す型
// どのNodeに何のstyleがついてるかをまとめてる型
#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Node, // pointer to a DOM node
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

type MatchedRule<'a> = (Specificity, &'a Rule);

//　stylesheetを全てのdomに適用してStyleNodeを返す
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            crate::dom::NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            crate::dom::NodeType::Text(_) => HashMap::new(),
        },
        // styletreeを再帰的に行なっている
        children: root
            .children
            .iter()
            .map(|child: &Node| style_tree(child, stylesheet))
            .collect(),
    }
}

// elementにstyleを適用させている？
// ElementDataはただのElementでstylesheetはrule(margin: auto;)とかのvec
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    // valuesはdeclarationが追加されていく
    let mut values: HashMap<String, Value> = HashMap::new();
    let mut rules: Vec<((usize, usize, usize), &Rule)> = matching_rules(elem, stylesheet);

    // 何かsortしてる。css.rsでもidとかclassでこれやった気がする
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    return values;
}

//　全てのCSSのruleからそれを持つelementを抽出する
// stylesheetはruleのvec
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    // match_ruleが通ったものだけ返す
    // match_ruleはNoneを返す場合があるので
    stylesheet
        .rules
        .iter()
        .filter_map(|rule: &Rule| match_rule(elem, rule))
        .collect()
}

// rule（例：.style{margin: auto;}）がelem(例：<h1 class="style">)とmatchしたらMatchRuleを返す。そうでなければ何も返さない。
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    //　selectorsはstyleを当てる対象
    // findは条件が合っているならselectorを返す
    rule.selectors
        .iter()
        .find(|selector: &&Selector| matches(elem, *selector))
        .map(|selector: &Selector| (selector.specificity(), rule))
}

//
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        crate::css::Selector::Simple(ref simple_selector) => {
            matches_simple_selector(elem, simple_selector)
        }
    }
}

//　SimpleSelectorはtag_name
fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // タグ(h1..)の名前が合ってなかったらそもそもfalse
    // selectorってvecじゃないけどうーん、わからん。Ruleならselectorsがvecだけど
    if selector
        .tag_name
        .iter()
        .any(|name: &String| elem.tag_name != *name)
    {
        return false;
    }

    // Check ID selector
    if selector.id.iter().any(|id: &String| elem.id() != Some(id)) {
        return false;
    }

    // Check class selectors
    let elem_classes: std::collections::HashSet<&str> = elem.classes();
    if selector
        .class
        .iter()
        .any(|class: &String| !elem_classes.contains(&**class))
    {
        return false;
    }

    // We didn't find any non-matching selector components.
    return true;
}
