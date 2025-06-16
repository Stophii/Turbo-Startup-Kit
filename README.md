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

In this example im going to name my file `state.rs` so my `mod` and `use` will look like this

```rust
mod state;
use state::*;
```

> **Tip:** You can save your project for real-time updates with `cmd + s` or `ctrl + s`. And you can reload it with `cmd + r` or `ctrl + r`! Troubleshoot and make games fast by saving and testing a lot!


## State Machine

lets add some substance to that second file and include a state machine in our project

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

Now lets get into adding some sprites! First off we need a sprites folder, this is as simple as making a new folder named `sprites` inside of the src folder of your project!

```rust

```

Over your `turbo::go!`.

The above code is using some of the values we initialized in GameState and calls them with the `state.` prefix before the **field**. The purpose of this example is to showcase the use of our initialized fields and how we can change them. If you don't understand everything that is happening, that is quite alright — just understand the basic principles we've covered so far and build on that. Those principles are:

- turbo::init is where we initialize a **field**. **Fields** have a _name_ and a _type_.

- turbo::go continually runs, so we can write macros and call our **fields** there.

- Trying to reference something in `GameState` can be done by using the format `state.name`.

Understanding and progressing is a journey, and the purpose of this guide is to help you take those first steps.

## Common Errors

The final thing to go over is just some basic bugs you might see in your terminal.

- `Expected ( ; , { } )` – This appears typically when you have a syntax error. Forgetting to close a macro with a `;` or initializing values in `GameState` and not using `,` are common ways to run into this. `if` statements open with `{` and close with `}` — this is another common cause.

- `Mismatched types` – Mismatched types typically happen when the system expects a value to be a certain type, and the value provided is not of that type. For example, if I initialized `decimal_number` as `0`, this would not be an `f32`, since it's missing the decimal point. The correct way to write it would be `0.0`.

- `Cannot borrow` – Borrowing is probably the first roadblock you'll hit as a new Rust dev. It happens when you try to use a value that's already being used (borrowed) elsewhere. This often shows up when you start making custom functions that use values from your state. For now, just know that borrowing exists and don’t let it slow you down — it gets clearer over time.

> **Tip:** Check out our [SDK](https://docs.turbo.computer/learn/guides/rust-basics) for more resources!

Thanks for the read and don't forget to check out our [Turbo-Tools](https://www.youtube.com/playlist?list=PL4thXl4CNv5IjPSx9-_f-F60zZlsORw2z)
