mod battle_list;
use ndarray::Array3;
use std::time::Instant;
use battle_list::BattleList;

fn main() {
    let battle_list_content: image::DynamicImage = image::open("content.png").unwrap();
    let battle_list_content_vec = battle_list_content.as_bytes().to_vec();
    let start_time = Instant::now();
    let content = Array3::from_shape_vec((998, 156, 4), battle_list_content_vec).unwrap();
    let battle_list = BattleList { content };
    let creatures_count = battle_list.get_creatures_count();
    println!("duration: {} ns", start_time.elapsed().as_nanos());
    println!("{:?}", creatures_count);
}