//lib.rs

mod state;
use state::*;

turbo::init! {
    struct GameState {
        screen: Screen,
        wizzy_tween: Tween<f32>,
    } = {
        Self {
            screen: Screen::Title,
            wizzy_tween: Tween::new(150.0)
                .duration(120)
                .ease(Easing::EaseOutCubic),

        }
    }
}

turbo::go! {
    let mut state = GameState::load();
    state_of_game(&mut state);
    state.save();
}