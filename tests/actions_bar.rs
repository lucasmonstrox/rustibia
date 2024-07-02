use rustibia::client::actions_bar::Action;
use rustibia::utils::image::load_image;

#[test]
fn should_method_is_equipped_return_false_when_action_is_empty() {
    let content = load_image("./assets/examples/client/actions_bar/empty_action.png").unwrap();
    let action = Action { content };
    assert_eq!(action.is_equipped(), false);
}
