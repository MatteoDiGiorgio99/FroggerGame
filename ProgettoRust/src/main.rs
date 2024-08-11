pub mod actor;
pub mod frogger;
pub mod pt2d;
pub mod rand;


fn main() {
    let mut game = frogger::FroggerGame::new(pt2d::pt(640, 448), 2, 5);
    for _ in 0..100 {
        game.tick(String::new());
        for b in game.actors() {
            println!("{:?}", b.pos());
        }
        println!();
    }
}
