use std::collections::HashMap;

use image::RgbImage;
use image_compare;
use wz_reader::util::resolve_base;
use wz_reader::WzNodeName;

mod collect_skill_icon;
mod collect_taming_icon;
mod get_icon;
mod macros;
use macros::macros::*;

const TAMING_MOB_PATH: &'static str = "Character/TamingMob";
const SKILL_PATH: &'static str = "Skill";

// usage:
//   cargo run -- "path/to/Base.wz"
//   cargo run -- "D:\Path\To\Base.wz"
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let base_path = args.get(1).expect("missing base path");
    println!("base path: {}", base_path);
    let base_node = resolve_base(&base_path, None).unwrap();
    println!("resolve base done");

    let taming_mob_node = get_node_from_path!(base_node, TAMING_MOB_PATH);
    let skill_node = get_node_from_path!(base_node, SKILL_PATH);

    let taming_mob_icon_map: HashMap<WzNodeName, RgbImage> =
        collect_taming_icon::collect_taming_icon(&taming_mob_node);
    println!("collect taming mob icon done");
    let skill_icon_map: HashMap<WzNodeName, RgbImage> =
        collect_skill_icon::collect_skill_icon(&skill_node);
    println!("collect skill icon done");

    let mut result_map: HashMap<WzNodeName, WzNodeName> = HashMap::new();

    for (taming_id, taming_icon) in taming_mob_icon_map.iter() {
        for (skill_id, skill_icon) in skill_icon_map.iter() {
            let res = image_compare::rgb_hybrid_compare(taming_icon, skill_icon);
            if res.ok().map(|r| r.score > 0.9) == Some(true) {
                result_map.insert(taming_id.clone(), skill_id.clone());
                break;
            }
        }
    }

    let mut list = result_map
        .into_iter()
        .map(|(k, v)| {
            (
                k.trim_start_matches("0")
                    .trim_end_matches(".img")
                    .to_string(),
                v.to_string(),
            )
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a.0.cmp(&b.0));

    println!("result: {:?}", list);

    Ok(())
}
