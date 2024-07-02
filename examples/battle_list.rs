use rustibia::client::battle_list::BattleList;
use rustibia::utils::image::load_image;

fn main() {
    let content = load_image("./assets/examples/client/battle_list/no_creatures.png").unwrap();
    let battle_list = BattleList { content };
    println!("no creatures");
    println!("count: {:?}", battle_list.get_creatures_count());
    println!(
        "max_creatures_count: {:?}",
        battle_list.get_max_creatures_count()
    );
    println!(
        "has_creature_in_target: {:?}",
        battle_list.has_creature_in_target()
    );
    println!("");

    let content = load_image("./assets/examples/client/battle_list/full_of_creatures.png").unwrap();
    let battle_list = BattleList { content };
    println!("full of creatures");
    println!("count: {:?}", battle_list.get_creatures_count());
    println!(
        "max_creatures_count: {:?}",
        battle_list.get_max_creatures_count()
    );
    println!(
        "has_creature_in_target: {:?}",
        battle_list.has_creature_in_target()
    );
    println!("");

    let content =
        load_image("./assets/examples/client/battle_list/target_is_in_first_position.png").unwrap();
    let battle_list = BattleList { content };
    println!("target in first position");
    println!("count: {:?}", battle_list.get_creatures_count());
    println!(
        "max_creatures_count: {:?}",
        battle_list.get_max_creatures_count()
    );
    println!(
        "has_creature_in_target: {:?}",
        battle_list.has_creature_in_target()
    );
    println!("");

    let content =
        load_image("./assets/examples/client/battle_list/target_is_in_last_position.png").unwrap();
    let battle_list = BattleList { content };
    println!("target in last position");
    println!("count: {:?}", battle_list.get_creatures_count());
    println!(
        "max_creatures_count: {:?}",
        battle_list.get_max_creatures_count()
    );
    println!(
        "has_creature_in_target: {:?}",
        battle_list.has_creature_in_target()
    );
    println!("");
}
