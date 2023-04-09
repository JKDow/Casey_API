
//tests that create game settings then confirm them to create a game admin
#[test]
fn test_game_admin() {
    use crate::setup::GameSettings;
    let settings = GameSettings::new(4, 2, 1);
    let _admin = settings.confirm_settings().unwrap();
} 