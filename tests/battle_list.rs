use rustibia::client::battle_list::BattleList;
use rustibia::utils::image::load_image;

#[test]
fn should_method_get_creatures_count_return_0_when_there_are_no_creatures() {
    let content = load_image("./assets/examples/client/battle_list/no_creatures.png").unwrap();
    let battle_list = BattleList { content };
    assert_eq!(battle_list.get_creatures_count(), 0);
}

#[test]
fn should_method_get_creatures_count_return_45_when_its_full_of_creatures() {
    let content = load_image("./assets/examples/client/battle_list/full_of_creatures.png").unwrap();
    let battle_list = BattleList { content };
    assert_eq!(battle_list.get_creatures_count(), 45);
}

#[test]
fn should_method_get_max_creatures_count_return_45() {
    let content = load_image("./assets/examples/client/battle_list/full_of_creatures.png").unwrap();
    let battle_list = BattleList { content };
    assert_eq!(battle_list.get_max_creatures_count(), 45);
}

#[test]
fn should_method_has_creature_in_target_return_false_when_there_are_no_creatures() {
    let content = load_image("./assets/examples/client/battle_list/no_creatures.png").unwrap();
    let battle_list = BattleList { content };
    assert_eq!(battle_list.has_creature_in_target(), false);
}

#[test]
fn should_method_has_creature_in_target_return_false_when_has_no_creature_in_target() {
    let content = load_image("./assets/examples/client/battle_list/full_of_creatures.png").unwrap();
    let battle_list = BattleList { content };
    assert_eq!(battle_list.has_creature_in_target(), false);
}

#[test]
fn should_method_has_creature_in_target_return_true_when_target_is_in_first_position() {
    let content =
        load_image("./assets/examples/client/battle_list/target_is_in_first_position.png").unwrap();
    let battle_list = BattleList { content };
    assert_eq!(battle_list.has_creature_in_target(), true);
}

#[test]
fn should_method_has_creature_in_target_return_true_when_target_is_in_last_position() {
    let content =
        load_image("./assets/examples/client/battle_list/target_is_in_last_position.png").unwrap();
    let battle_list = BattleList { content };
    assert_eq!(battle_list.has_creature_in_target(), true);
}
