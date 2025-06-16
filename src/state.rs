//state.rs

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub enum Screen {
    Title,
    Game,
}


pub fn state_of_game(state: &mut GameState) {
    match state.screen {
        Screen::Title => {
          clear(0xffffffff);
	        if gamepad(0).a.just_pressed() {
		        state.screen = Screen::Game;
	        }
            text!("this is the title!", color = 0x000000ff);
        },
        Screen::Game => {
            clear(0x000000ff);
            let p = pointer();
            let canvas_bounds = bounds::canvas();

        	if gamepad(0).a.just_pressed() {
		        state.screen = Screen::Title;
		    }

            let x = state.wizzy_tween.get();
            text!("this is the game!", color = 0xffffffff);
            sprite!("Wizzy", x = x, y = 45);

            let crab = animation::get("crab");

            sprite!(
                animation_key = "crab",
                default_sprite = "Decapod#Idle",
            );

            if state.wizzy_tween.done() {
                state.wizzy_tween.set(150.0);
            }

            let button = Bounds::with_size(48, 14)
                .anchor_center(&canvas_bounds)
                .translate_y(16);
            
            let is_btn_hovered = p.xy().intersects_bounds(button);

            let (regular_color, hover_color, pressed_color) = (0x33CCFFff, 0x66DDFFFF, 0x00FFFFFF);

            let button_color = if p.pressed() && is_btn_hovered {
                pressed_color
            } else if is_btn_hovered {
                hover_color
            } else {
                regular_color
            };

            rect!(
                color = button_color,
                xy = button.xy(),
                wh = button.wh(),
                border_radius = 2,
            );

            let label_bounds = button.inset_left(4).inset_top(4);
            text!("Attack!", x = label_bounds.x(), y = label_bounds.y(), font = "medium");

            if is_btn_hovered && p.just_pressed() {
                crab.use_sprite("Decapod#Ranged Attack");
                crab.set_repeat(1);
                state.wizzy_tween.set(200.0);
            }
        },
    }
}