use casey_api_0::*;

fn main() {

    let mut game = Game::new(3);
    game.deal(10);
    prints::print_hand(game.get_hand());
    game.draw_deck();
    let hand = game.get_hand(); 
    
}