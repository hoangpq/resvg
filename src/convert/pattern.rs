// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use svgdom::{
    self,
    FuzzyEq,
};

// self
use tree;
use short::{
    AId,
};
use math::*;
use traits::{
    GetValue,
    GetViewBox,
};
use {
    Options,
};


pub fn convert(
    node: &svgdom::Node,
    opt: &Options,
    rtree: &mut tree::RenderTree,
) {
    let ref attrs = node.attributes();

    let rect = convert_rect(attrs);
    debug_assert!(!rect.width().is_fuzzy_zero());
    debug_assert!(!rect.height().is_fuzzy_zero());

    rtree.append_node(tree::DEFS_DEPTH, tree::NodeKind::Pattern(tree::Pattern {
        id: node.id().clone(),
        units: super::convert_element_units(&attrs, AId::PatternUnits),
        content_units: super::convert_element_units(&attrs, AId::PatternContentUnits),
        transform: attrs.get_transform(AId::PatternTransform).unwrap_or_default(),
        rect,
        view_box: node.get_viewbox().ok(),
    }));

    super::convert_nodes(node, opt, tree::DEFS_DEPTH + 1, rtree);
}

pub fn convert_rect(attrs: &svgdom::Attributes) -> Rect {
    Rect::from_xywh(
        attrs.get_number(AId::X).unwrap_or(0.0),
        attrs.get_number(AId::Y).unwrap_or(0.0),
        attrs.get_number(AId::Width).unwrap_or(0.0),
        attrs.get_number(AId::Height).unwrap_or(0.0),
    )
}