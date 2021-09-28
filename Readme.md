Pong a classic first game to build, mostly followed the amethyst tutorial https://book.amethyst.rs/book/stable/pong-tutorial  
with a lot of UI and structural changes.

The game is simple, face off against the AI, move your paddle with W and S and don't let the ball touch the left side of the arena.

To run, either download the release for your target platform (Except if you're on Mac, then run from source).


Run from source:
1. Install Rust
2. Clone this repo
3. (for Mac only) change "vulkan" to "metal" in cargo.toml under amethyst features.
4. Enter the repo directory
5. Run cargo build --release
6. Have fun!
