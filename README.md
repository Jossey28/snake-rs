# Snake-Rs.

## 1997 Snake built in Rust

### Current directory layout

There are currently six important project files
```pwsh
Cargo.toml

examples/pixels_test.rs

src/event_handler.rs
src/game_logic.rs
src/main.rs
src/ui.rs
```
#### `examples/`

`pixels_test.rs` was created to assist in my understanding of ratatui.

Any further files in `examples/` are only for personal development and have no affect on gameplay

#### `src/`

Where all the source code for snake functionality lays. 

##### `src/main.rs`

Contains the code needed to initialize the program, defines and creates the `App` struct the snake games uses for state-management. 

##### `src/game_logic.rs`

This files handles the independent objects needed to have the game (e.g the snake, food, etc) it also defines how they're displayed on the screen as well as game settings 

##### `src/event_handler.rs`

This code is pretty much ripped from the ratatui tutorial, it contains an implementation for non-threadblocking input event handling using channels. I'm not too confident I understand the underlying concepts, will need to revisit.

##### `src/ui.rs`

As of right now, this file contains the code needed to display the title screen. In the future, it will contain the code needed to show the score counter as well.

##
### Running it locally
```bash
git clone https://github.com/Jossey28/snake-rs.git

cd snake-rs

cargo build --release

.\target\release\snake-rs.exe
```

##
### Project Goals

The purpose of this project was to serve as an introduction to the Rust programming language and its culture as well as the Ratatui Terminal-User-Interface library.

##
### Project Update

The app in its current state has served its purpose of introducing me to the core concepts of rust and how it is supposed to be programmed, so I will be putting a stop here.

I can see myself returning to this project and turning it into a fully-functional game, however my primary interest doesn't lie in game design, so I will be moving on.