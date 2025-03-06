use wasm_bindgen::prelude::*;
use bracket_lib::prelude::*;

// Module declarations
pub mod components;
pub mod player;
pub mod rect;
pub mod map;
pub mod game_state;

#[wasm_bindgen(start)]
pub fn wasm_main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .with_fps_cap(30.0)
        .build()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let gs = game_state::State::new();
    main_loop(context, gs)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}