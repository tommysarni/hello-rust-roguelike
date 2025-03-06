use bracket_lib::prelude::*;
use hellorust::game_state::State;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    
    let gs = State::new();
    main_loop(context, gs)
}