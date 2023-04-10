
//tests that create game settings then confirm them to create a game admin
#[test]
fn game_admin() {
    use crate::setup::GameSettings;
    let settings = GameSettings::new(4, 2, 1, 11);
    let _admin = settings.confirm_settings().unwrap();
} 

#[test]
fn game_admin_invalid_team_size() {
    use crate::setup::GameSettings;
    let settings = GameSettings::new(4, 0, 1, 11);
    let admin = settings.confirm_settings();
    assert!(admin.is_err());
}

#[test]
fn player_num_team_num() {
    //test that creates a game admin with 4 players and 2 teams and checks each player has the right number
    use crate::setup::GameSettings;
    use crate::game::players::Player;
    let settings = GameSettings::new(4, 2, 1, 11);
    let mut admin = settings.confirm_settings().unwrap();
    let mut players: Vec<Player> = vec![]; 
    for i in 0..4 {
        players.push(admin.take_player(i).unwrap());
    }
    for i in 0..4 {
        assert_eq!(players[i].player_num, i as u8);
        //check team number, should alternate between 0 and 1
        assert_eq!(players[i].team_num, i as u8 % 2);
    }
    
}