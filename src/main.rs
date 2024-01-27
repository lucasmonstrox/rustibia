mod battle_list;
mod game_window;
use ndarray::Array2;
use std::time::Instant;
// use battle_list::get_creatures_count;
use game_window::get_creatures_bars;

fn main() {
    // let battle_list_content: image::DynamicImage = image::open("content.png").unwrap();
    // let battle_list_content_vec = battle_list_content.grayscale().to_luma8().to_vec();
    // let battle_list_content_array2 = Array2::from_shape_vec((998, 156), battle_list_content_vec).unwrap();
    // let creatures_count = get_creatures_count(&battle_list_content_array2);
    // let duration = start_time.elapsed();
    // println!("creatures count {}", creatures_count);
    let game_window: image::DynamicImage = image::open("game_window.png").unwrap();
    let game_window_vec = game_window.grayscale().to_luma8().to_vec();
    let game_window_array = Array2::from_shape_vec((704, 960), game_window_vec).unwrap();
    let start_time = Instant::now();
    let bars = get_creatures_bars(&game_window_array);
    let duration = start_time.elapsed();
    println!("duration: {} ns", duration.as_nanos());
    println!("{:?}", bars);
}