use casey_api_0::game::*;

fn main() {
    let mut game = Game::new(2);
    game.deal(11);
    let x = game.get_hand();
}