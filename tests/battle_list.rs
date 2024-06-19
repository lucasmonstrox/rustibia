use ndarray::ArrayView3;
use rustibia::client::battle_list::BattleList;

#[test]
fn return_45_when_battle_list_is_full_of_creatures() {
    let content_vec = image::open("content.png").unwrap().as_bytes().to_vec();
    let content = ArrayView3::from_shape((998, 156, 4), &content_vec).unwrap();
    let battle_list = BattleList { content };

    assert_eq!(battle_list.get_creatures_count(), 45);
}
