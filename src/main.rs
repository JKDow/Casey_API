use casey_api_0::*;


fn main() {

    let mut game = Game::new(2);
    game.deal(10);
    game.print_hand();
    game.draw_deck();
    println!("");
    game.print_hand();
    println!("");
    game.throw(2); 
    game.print_hand();

}