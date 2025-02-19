use macroquad::prelude::*;

pub trait StellarObject {
    fn get_position(&self) -> Vec2;
    fn get_speed(&self) -> Vec2;
    fn get_size(&self) -> f32;

    fn angle_from_mouse(&self) -> Vec2;

    fn set_position(&mut self, position: Vec2);
    fn bound_to(coord: f32, max: f32) -> f32;
    fn bound_pos(pos: Vec2) -> Vec2;
}

#[macro_export]
macro_rules! impl_stellar_object {
    ($t:ty) => {
        impl StellarObject for $t {
            // Getters
            fn get_position(&self) -> Vec2 {
                self.position
            }

            fn get_speed(&self) -> Vec2 {
                self.speed
            }

            fn get_size(&self) -> f32 {
                self.size
            }

            // Setters

            fn set_position(&mut self, position: Vec2) {
                self.position = position;
            }

            // Position calculation

            fn bound_pos(pos: Vec2) -> Vec2 {
                let mut pos = pos;
                pos.x = if cfg!(test) {
                    pos.x
                } else {
                    Self::bound_to(pos.x, screen_width())
                };
                pos.y = if cfg!(test) {
                    pos.y
                } else {
                    Self::bound_to(pos.y, screen_height())
                };
                pos
            }

            fn bound_to(coord: f32, max: f32) -> f32 {
                if coord < 0.0 {
                    max - coord
                } else if coord > max {
                    coord - max
                } else {
                    coord
                }
            }
            fn angle_from_mouse(&self) -> Vec2 {
                let mouse_pos = mouse_position();
                let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);

                let dx = mouse_vec.x - self.position.x;
                let dy = mouse_vec.y - self.position.y;

                let res = dy.atan2(dx);

                vec2(res.cos(), res.sin())
            }
        }
    };
}
