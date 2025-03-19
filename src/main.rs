use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await;
    }
}
