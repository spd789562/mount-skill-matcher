use image::DynamicImage;
use wz_reader::{property::png::get_image, WzNodeArc};

pub fn get_icon(node: &WzNodeArc) -> Option<DynamicImage> {
    let icon_node = node
        .read()
        .unwrap()
        .at("icon")
        .or_else(|| node.read().unwrap().at_path("info/icon"));
    icon_node.and_then(|icon_node| get_image(&icon_node).ok())
}
