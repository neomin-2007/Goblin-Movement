use macroquad::prelude::*;
mod player;

#[macroquad::main("Test Game")]
async fn main() {

    let mut user  = player::PlayerData {
        x: 10.0,
        y: 15.0,
        texture: load_texture("sprites/orc.png").await.unwrap(),
        fliped_x: true,
    };
    
    loop {
        clear_background(WHITE);
        user.action();
        next_frame().await
    }
}