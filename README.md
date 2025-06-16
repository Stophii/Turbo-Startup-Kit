# Turbo Startup Kit

## Description

A walkthrough on some Rust syntax and Turbo macros!

> **Tip** This [video](https://youtu.be/g6EFmxjdR_o) goes over this written guide

## What You'll Need

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

## Initialization and Types

Go ahead and paste this over the `turbo::init!` we just set up:

> **Tip:** You can save your project for real-time updates with `cmd + s` or `ctrl + s`.

```rust
turbo::init! {
    struct GameState {
        //name: type,
        decimal_number: f32,
        positive_number: u32,
        negative_number: i32,
        true_or_false: bool,
        words: String,
        list_of_decimal_numbers: Vec<f32>,
        list_of_words: Vec<String>,
        optional_positive_number: Option<u32>,
    } = {
        Self {
            //name: value,
            decimal_number: 0.0,
            positive_number: 1,
            negative_number: -1,
            true_or_false: false,
            words: "Hello, World!".to_string(),
            list_of_decimal_numbers: Vec::new(),
            list_of_words: Vec::new(),
            optional_positive_number: None,
        }
    }
}
```

For each **field** I have in my struct, I have to give it a _name_ and a _type_. When I look at the first **field**, it is named `decimal_number` and its _type_ is an `f32`, or a _decimal number_. I’ve named all of the fields appropriately to their type.

Down below we can see `Self {`, which is where we give the field its value specific to the type we set in `GameState {`. Let’s use the first, `decimal_number`, as an example.

```rust
    struct GameState {
        //name: type,
        decimal_number: f32,
        // other gamestate fields
    } = {
        Self {
            //name: value,
            decimal_number: 0.0,
            // other gamestate fields
```

`decimal_number` was set as an `f32` and then initialized to `0.0`. Initializing fields correctly is one of the first things to hone in on in Rust. This process is called initialization — hence the name `Turbo::init!`. This runs once at the beginning of your project to set up its initial values.

## Basic Turbo Macros

Some basic macros you can find a lot of mileage with in Turbo are `rect`, `ellipse`, `text`, `log`, and `gamepad`.

Drawing boxes and text can help you set up the beginning title screen or menus for your game. Let’s add in those macros into our go loop:

> **Tip:** If you are starting out, try using `just_pressed` instead of `pressed` for gamepad input!

Replace this:

```rust
turbo::go! {
    let mut state = GameState::load();

    state.save();
}
```

with:

```rust
turbo::go! {
    let mut state = GameState::load();

    rect!(x = 10, y = 20, w = 10, h = 20, color = 0xffffffff);
    ellipse!(x = 30, y = 30, w = 10, h = 20, color = 0xffffffff);
    text!("This is a sentence!\nThis is a new line for my sentence...", x = 40, y = 5, font = "medium", color = 0xffffffff);
    if gamepad(0).a.just_pressed() {
        // do something here!
    }
    if gamepad(0).b.just_pressed() {
        // do something here!
    }
    if gamepad(0).start.just_pressed() {
        // do something here!
    }
    if gamepad(0).up.just_pressed() {
        // do something here!
    }
    log!("{}", state.words);

    state.save();
}
```

If you save your project now, you can see it update in real time with the rectangle, ellipse, and text we just added.

If you check your terminal, you'll also be able to see a log!

> **Tip** Check out more Turbo Macros [here](https://docs.turbo.computer/learn/getting-started) in the API section

## Mastering Turbo

Writing Rust and blending that with Turbo's powerful macros will give you mastery over _**making games fast**_.

From here on, you have the building blocks to get started. But if you'd like to learn a few more things, keep following along!

Paste this block:

```rust
turbo::go! {
    let mut state = GameState::load();

    if gamepad(0).a.just_pressed() {
        state.true_or_false = !state.true_or_false;
        log!("value changed from {} to {}", !state.true_or_false, state.true_or_false)
    }
    if gamepad(0).b.pressed() {
        rect!(x = 128, y = 72, w = state.positive_number, h = state.decimal_number, color = 0xffffffff);
    }
    if gamepad(0).start.just_pressed() {
        state.list_of_decimal_numbers.push(1.1);
        state.list_of_decimal_numbers.push(2.2);
        state.list_of_decimal_numbers.push(3.3);
        log!("added the numbers!");
    }
    if gamepad(0).up.just_pressed() {
        state.decimal_number += 1.2;
        state.positive_number += 1;
        state.negative_number -= 1;
        log!("my numbers have increased!")
    }

    let example = format!("decimal_number:{}\npositive_number:{}\nnegative_number:{}\nbool:{}", state.decimal_number, state.positive_number, state.negative_number, state.true_or_false);
    text!(&example, x = 40, y = 5, font = "medium", color = 0xffffffff);

    let vector = format!("decimal list: {:?}", state.list_of_decimal_numbers);
    text!(&vector, x = 40, y = 55, font = "medium", color = 0xffffffff);

    state.save();
}
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
