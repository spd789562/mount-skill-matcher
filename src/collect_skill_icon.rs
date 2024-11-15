use std::collections::HashMap;

use image::RgbImage;
use wz_reader::{util::node_util, WzNodeArc, WzNodeName};

use super::get_icon::get_icon;

pub fn collect_skill_icon(node: &WzNodeArc) -> HashMap<WzNodeName, RgbImage> {
    let mut map = HashMap::new();

    let childrens = node.read().unwrap().children.clone();

    for (folder_id, folder_node) in childrens.iter() {
        if !folder_id.starts_with("800") || !folder_id.ends_with(".img") {
            continue;
        }
        let _ = node_util::parse_node(folder_node);

        let skill_nodes = folder_node
            .read()
            .unwrap()
            .at("skill")
            .unwrap()
            .read()
            .unwrap()
            .children
            .clone();

        for (id, skill_node) in skill_nodes.iter() {
            if let Some(image) = get_icon(&skill_node) {
                map.insert(id.clone(), image.into());
            }
        }
        folder_node.write().unwrap().unparse();
    }
    map
}
