use macroquad::prelude::*;

/// Structure du trou noir.
pub struct BlackHole {
    pub position: Vec2,
    pub size: f32,
    pub rotation: f32,
}

impl BlackHole {
    const BLACKHOLE_SIZE: f32 = 50.0;

    /// Créer un nouveau trou noir.
    ///     
    /// # Arguments
    /// - `position` - Position du trou noir.
    ///     
    /// # Returns
    /// - `BlackHole` Nouveau trou noir.
    ///
    /// # Examples
    ///
    /// ```
    /// let blackhole = BlackHole::new(Vec2::new(100.0, 100.0));
    /// assert_eq!(blackhole.position, Vec2::new(100.0, 100.0));
    /// ```
    ///
    pub fn new(position: Vec2) -> Self {
        BlackHole {
            position,
            size: Self::BLACKHOLE_SIZE,
            rotation: 0.0,
        }
    }

    /// Getter de la position du trou noir.
    ///     
    /// # Returns
    /// - `Vec2` Position du trou noir.
    ///
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Getter de la taille du trou noir.
    ///     
    /// # Returns
    /// - `f32` Taille du trou noir.
    ///
    pub fn get_size(&self) -> f32 {
        self.size
    }

    /// Met à jour la rotation du trou noir (tourne sur lui-même).
    pub fn update_rotation(&mut self) {
        self.rotation += 0.01;
    }

    /// Dessine le trou noir à partir de sa texture (son skin)
    ///     
    /// # Arguments
    /// - `texture` - Texture du trou noir.
    ///     
    pub fn draw_blackhole(&self, texture: &Texture2D) {
        draw_texture_ex(
            texture,
            self.get_position().x - (self.get_size() / 2.0),
            self.get_position().y - (self.get_size() / 2.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.get_size() * 2.0, self.get_size() * 2.0)),
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }
}
