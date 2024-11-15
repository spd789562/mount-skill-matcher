use std::collections::HashMap;

use image::RgbImage;
use wz_reader::{util::node_util, WzNodeArc, WzNodeName};

use super::get_icon::get_icon;

pub fn collect_taming_icon(node: &WzNodeArc) -> HashMap<WzNodeName, RgbImage> {
    let mut map = HashMap::new();

    let childrens = node.read().unwrap().children.clone();

    for (id, img_node) in childrens.iter() {
        if id.starts_with("0191") || id.starts_with("0198") || !id.ends_with(".img") {
            continue;
        }
        let _ = node_util::parse_node(img_node);

        if let Some(image) = get_icon(&img_node) {
            map.insert(id.clone(), image.into());
        }

        img_node.write().unwrap().unparse();
    }
    map
}
