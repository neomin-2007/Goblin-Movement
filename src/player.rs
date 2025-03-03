use macroquad::prelude::*;

pub struct PlayerData {
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
    pub fliped_x: bool,
}

impl PlayerData {

    pub fn action(&mut self) {

        if is_key_down(KeyCode::A) {
            self.move_x(-2.0);
            self.fliped_x = false;
        }
        if is_key_down(KeyCode::D) {
            self.move_x(2.0);
            self.fliped_x = true;
        }
        if is_key_down(KeyCode::W) {
            self.move_y(-2.0);
        }
        if is_key_down(KeyCode::S) {
            self.move_y(2.0);
        }
    

        draw_texture_ex(&self.texture, self.x, self.y, WHITE, DrawTextureParams {
            flip_x: self.fliped_x,
            ..Default::default()
        });
    }

    pub fn move_x(&mut self, amount: f32) {
        self.x = self.x + amount;
    }

    pub fn move_y(&mut self, amount: f32) {
        self.y = self.y + amount;
    }

}