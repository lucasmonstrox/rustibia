use rustibia::client::battle_list::BattleList;
use rustibia::utils::image::load_image;

#[test]
fn return_0_when_battle_list_is_empty() {
    let content = load_image("./assets/examples/client/battle_list/empty_battle_list.png").unwrap();
    let battle_list = BattleList { content };
    assert_eq!(battle_list.get_creatures_count(), 0);
}

#[test]
fn return_45_when_battle_list_is_full_of_creatures() {
    let content = load_image("./assets/examples/client/battle_list/full_battle_list.png").unwrap();
    let battle_list = BattleList { content };
    assert_eq!(battle_list.get_creatures_count(), 45);
}
