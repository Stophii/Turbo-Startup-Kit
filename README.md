# Turbo Startup Kit

## Description

Follow along and get started making games fast with these useful tools!

## Some Basic Structure

Initialize a Turbo project with `turbo init projectname`. You can replace `projectname` with whatever you desire. After that, run your project with `turbo run -w projectname`.

Then replace the default project contents:

```rust
// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! ({
    text!("Hello, world!!!");
});
```

with this:

```rust
turbo::init! {
    struct GameState {
        //stuff goes here
    } = {
        Self {
            //stuff goes here
        }
    }
}

turbo::go! {
    let mut state = GameState::load();

    state.save();
}
```
Now we have a basic template structured with `turbo::init!` and `turbo::go!`.

> **Tip:** I'd recommend downloading the extension, [Rust Analyzer](https://marketplace.visualstudio.com/items/?itemName=rust-lang.rust-analyzer).

## More than Lib.rs

Whenever you make games rarely will they be just one file. Adding multiple files is easy with turbo. Let's create a new file and add this line to the top of it:

```rust
use crate::*;
```

And in order to use this new file we'll need to add these lines to the top of our `lib.rs`

```rust
mod filename;
use filename::*;
```

By adding mod and use to the top of your file, you can utilize your new file seamlessly

In this example I'm going to name my file `state.rs` so my `mod` and `use` will look like this

```rust
mod state;
use state::*;
```

> **Tip:** You can save your project for real-time updates with `cmd + s` or `ctrl + s`. And you can reload it with `cmd + r` or `ctrl + r`! Troubleshoot and make games fast by saving and testing a lot!


## State Machine

let's add some substance to that second file and include a state machine in our project

```rust
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub enum Screen {
    Title,
    Game,
}
```
Once we have the enum in we can add in our `state_of_game` function

```rust
pub fn state_of_game(state: &mut GameState) {
    match state.screen {
        Screen::Title => {
          clear(0xffffffff);
	        if gamepad(0).a.just_pressed() {
		        state.screen = Screen::Game;
	        }
        },
        Screen::Game => {
          clear(0x000000ff);
        	if gamepad(0).a.just_pressed() {
		        state.screen = Screen::Title;
		    }
        },
        _ => {
        
        }
    }
}
```

This is not going to work until we add in our `screen` to our `gamestate` in `lib.rs`

```rust
turbo::init! {
    struct GameState {
        screen: Screen,

    } = {
        Self {
            screen: Screen::Title,

        }
    }
}
```

and finally we add in our function to run in the `turbo::go!` loop

```rust
turbo::go! {
    let mut state = GameState::load();
    state_of_game(&mut state);
    state.save();
}
```

> **Tip:** You can check out the state machine Turbo-Tool [here](https://www.youtube.com/watch?v=6XMg5csFccw&t=1s)!

## Adding Sprites

Now let's get into adding some sprites! First off we need a sprites folder, this is as simple as making a new folder named `sprites` right alongside your src folder.

Sprites added to this folder in the file format of: `.png`, `.bmp`, `.jpg/.jpeg`, `.gif`, `.webp`, and `.ase/.aseprite` will be able to be used!

> **Tip:** You can make a `fonts` and `audio` folder in the same place to add sounds or sound effects as well as fonts!

```rust
sprite!("Decapod#Idle");
sprite!("Wizzy", x = 150, y = 45);
```

Sprites made in Aesprite will need to have their appropriate tag referenced, in this scenario I am choosing to display the idle animation with the `#Idle` tag.

If you wanted to use inputs to change your sprite you can use an animation key and access animation commands

```rust
    let crab = animation::get("crab");

    sprite!(
	animation_key = "crab",
	default_sprite = "Decapod#Idle",
    );

    if gamepad(0).right.just_pressed() {
	crab.use_sprite("Decapod#Ranged Attack");
	crab.set_repeat(1);
    }
```

now if I hit the right arrow key or D on the keyboard my sprite will use it's `#Ranged Attack` and then go back to `#Idle`

> **Tip:** Check out the sprite SDK for more info [here](https://docs.turbo.computer/learn/api/sprites)! 


## Power of Tween

Now we have some sprites but nothing moves other than the animations I have. let's go ahead and add some tweening so the wizzy dodged the attack!

First let's add a `Tween` into the `Gamestate`

```rust
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
```

with `wizzy_tween` we just need to go change our x position of the wizzy sprite to the `wizzy_tween` we can do that with `.get()`


```rust
let x = state.wizzy_tween.get();

```

and update the sprite to match

```rust
sprite!("Wizzy", x = x, y = 45);

```

and let's just use our gamepad input as the trigger to change the position

```rust
    if gamepad(0).right.just_pressed() {
	crab.use_sprite("Decapod#Ranged Attack");
	crab.set_repeat(1);
	state.wizzy_tween.set(200.0); //<-- add this
    }
```

now the wizzy dodges the attack!

If we wanted we can reset him like this

```rust
    if state.wizzy_tween.done() {
	state.wizzy_tween.set(150.0);
    }
```

## Pushing the Bounds

Alright now for the grand finale let's add in a button we can click with the mouse. We'll use `bounds` to accomplish this. let's start off by adding in our `pointer` and our `canvas_bounds`

```rust
    let p = pointer();
    let canvas_bounds = bounds::canvas();
```

Now let's build on this by adding in our button with `bounds`!

```rust
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
```

and finally let's add a reaction for clicking this button

```rust
    if is_btn_hovered && p.just_pressed() {
	crab.use_sprite("Decapod#Ranged Attack");
	crab.set_repeat(1);
	state.wizzy_tween.set(200.0);
    }
```

Now instead of key inputs we can just press the button to try to attack the Wizzy!

Thanks for following along, hopefully you understand how to use a `State Machine`, add a `Sprite`, `Tween` an object, and use `Bounds`!

>**Tip** Join our [discord](discord.gg/V5YWWvQvKW) and message `Stophy` for questions!
