use rustibia::client::action_bars::Action;
use rustibia::utils::image::load_image;

fn main() {
    let content = load_image("./assets/examples/client/action_bars/action/empty.png").unwrap();
    let action: Action = Action { content };
    println!("empty action");
    println!("get_count: {:?}", action.get_count().unwrap_or(0));
    println!("get_hotkey: {:?}", action.get_hotkey().unwrap_or(""));
    println!("get_item_name: {:?}", action.get_item_name().unwrap_or(""));
    println!("is_equipped: {:?}", action.is_equipped());
    println!("is_usable: {:?}", action.is_usable());
    println!("");

    let content = load_image("./assets/examples/client/action_bars/action/equipped.png").unwrap();
    let action: Action = Action { content };
    println!("equipped action");
    println!("get_count: {:?}", action.get_count().unwrap_or(0));
    println!("get_hotkey: {:?}", action.get_hotkey().unwrap_or(""));
    println!("get_item_name: {:?}", action.get_item_name().unwrap_or(""));
    println!("is_equipped: {:?}", action.is_equipped());
    println!("is_usable: {:?}", action.is_usable());
    println!("");
}
