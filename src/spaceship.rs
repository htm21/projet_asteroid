use crate::bind::Bind;
use crate::blackhole::BlackHole;
use crate::impl_stellar_object;
use crate::sound::Sound;
use crate::stellarobject::StellarObject;
use crate::utils::has_even_decimal_part;
use crate::Asteroid;
use macroquad::prelude::*;

/// Représente le vaisseau spatial contrôlé par le joueur.
///
/// Le vaisseau spatial peut se déplacer, entrer en collision avec des objets
/// comme des astéroïdes ou des trous noirs, et possède un bouclier qui peut
/// diminuer après une collision.
pub struct Spaceship {
    /// Position actuelle du vaisseau spatial.
    position: Vec2,
    /// Vitesse actuelle du vaisseau spatial.
    speed: Vec2,
    /// Taille du vaisseau spatial.
    size: f32,
    /// Rotation actuelle du vaisseau spatial, en radians.
    rotation: f32,
    /// Bouclier du vaisseau (nombre de vies restantes).
    pub shield: u32,
    /// Temps de la dernière collision, en secondes.
    last_collision_time: f64,
    /// Score actuel du joueur.
    pub score: u32,
}

impl Spaceship {
    /// Taille constante du vaisseau spatial.
    pub const SPACE_SHIP_SIZE: f32 = 20.0;
    /// Accélération du vaisseau spatial.
    const SPACE_SHIP_ACCELERATION: f32 = 0.18;
    /// Décélération appliquée à chaque mise à jour.
    const SPACE_SHIP_DECELERATION: f32 = 0.99;

    /// Crée une nouvelle instance de `Spaceship` avec des paramètres par défaut.
    ///
    /// # Retourne
    ///
    /// Un vaisseau spatial initialisé au centre de l'écran avec un bouclier de 3.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// let spaceship = Spaceship::new();
    /// ```
    pub fn new() -> Self {
        Self {
            position: if cfg!(test) {
                Vec2::new(220.0, 330.0)
            } else {
                Self::starting_pos()
            }, // Pour éviter l'erreur 'THREAD_ID.is_some()' lors de l'éxécution de tests.
            speed: Vec2::new(0.0, 0.0),
            size: Self::SPACE_SHIP_SIZE,
            rotation: 0.0,
            shield: 3,
            last_collision_time: -2.0,
            score: 0,
        }
    }
    /// Retourne la position de départ du vaisseau spatial (centre de l'écran).
    fn starting_pos() -> Vec2 {
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
    }
    /// Retourne la rotation actuelle du vaisseau spatial en radians.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// let rotation = spaceship.get_rotation();
    /// ```
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    /// Vérifie si le vaisseau spatial est en collision avec un astéroïde.
    ///
    /// # Arguments
    ///
    /// - `asteroid`: Une référence à l'astéroïde à tester.
    ///
    /// # Retourne
    ///
    /// - `true` si le vaisseau spatial est en collision avec l'astéroïde.
    /// - `false` sinon.
    pub fn collision(&self, asteroid: &Asteroid) -> bool {
        let distance = Vec2::distance(self.get_position(), asteroid.get_position());
        let sum_radius = self.get_size() + asteroid.get_size();
        distance < sum_radius
    }
    /// Vérifie les collisions avec une liste d'astéroïdes.
    ///
    /// # Arguments
    ///
    /// - `asteroids`: Une liste mutable d'astéroïdes.
    ///
    /// # Retourne
    ///
    /// - `true` si une collision est détectée.
    /// - `false` sinon.
    ///
    /// # Note
    ///
    /// Cette méthode est asynchrone pour être utilisée dans des boucles de jeu.
    pub async fn spaceship_collision(&self, asteroids: &mut [Asteroid]) -> bool {
        // Check if the spaceship collides, repeated at each frame
        let mut i = 0;
        while i < asteroids.len() {
            if self.collision(&asteroids[i]) {
                return true;
            }
            i += 1;
        }
        false
    }
    /// Vérifie si le vaisseau spatial est en collision avec un trou noir.
    ///
    /// # Arguments
    ///
    /// - `blackhole`: Une référence au trou noir à tester.
    ///
    /// # Retourne
    ///
    /// - `true` si une collision est détectée.
    /// - `false` sinon.
    pub fn collision_blackhole(&self, blackhole: &BlackHole) -> bool {
        let distance = Vec2::distance(self.get_position(), blackhole.get_position());
        let sum_radius = self.get_size() + blackhole.get_size();
        distance < sum_radius
    }
    /// Vérifie les collisions avec une liste de trous noirs.
    ///
    /// # Arguments
    ///
    /// - `blackholes`: Une liste mutable de trous noirs.
    ///
    /// # Retourne
    ///
    /// - `true` si une collision est détectée.
    /// - `false` sinon.
    ///
    /// # Note
    ///
    /// Cette méthode est asynchrone pour être utilisée dans des boucles de jeu.
    pub async fn spaceship_blackhole_collision(&self, blackholes: &mut [BlackHole]) -> bool {
        // Check if the spaceship collides, repeated at each frame
        let mut i = 0;
        while i < blackholes.len() {
            if self.collision_blackhole(&blackholes[i]) {
                return true;
            }
            i += 1;
        }
        false
    }
    /// Vérifie et met à jour l'état du bouclier après une collision.
    ///
    /// # Arguments
    ///
    /// - `current_time`: Le temps actuel (en secondes).
    /// - `sound`: Une référence à l'objet `Sound` pour jouer un effet sonore.
    ///
    /// # Retourne
    ///
    /// - `true` si le bouclier est épuisé.
    /// - `false` sinon.
    pub fn check_shield(&mut self, current_time: f64, sound: &Sound) -> bool {
        if current_time - self.last_collision_time >= 2.0 {
            sound.play_sound_effect("collision");
            self.shield -= 1;
            self.last_collision_time = current_time;

            if self.shield == 0 {
                return true;
            }
        }
        false
    }
    /// Met à jour la position du vaisseau spatial en mode classique.
    ///
    /// # Arguments
    ///
    /// - `keybinding`: Les touches associées au contrôle du vaisseau.
    pub fn update_position_classic(&mut self, keybinding: &Bind) {
        self.speed += match (
            is_key_down(keybinding.get_up()),
            is_key_down(keybinding.get_down()),
        ) {
            (true, false) => {
                Vec2::new(self.rotation.sin(), -self.rotation.cos()) * Self::SPACE_SHIP_ACCELERATION
            }
            (false, true) => {
                Vec2::new(-self.rotation.sin(), self.rotation.cos()) * Self::SPACE_SHIP_ACCELERATION
            }
            _ => Vec2::new(0.0, 0.0),
        };
        self.speed *= Self::SPACE_SHIP_DECELERATION;

        self.rotation += match (
            is_key_down(keybinding.get_left()),
            is_key_down(keybinding.get_right()),
        ) {
            (true, false) => -0.05,
            (false, true) => 0.05,
            _ => 0.0,
        };

        self.position += self.speed;
        self.position = Spaceship::bound_pos(self.position);
    }
    /// Met à jour la position du vaisseau spatial en mode moderne.
    ///
    /// # Arguments
    ///
    /// - `keybinding`: Les touches associées au contrôle du vaisseau.
    pub fn update_position_modern(&mut self, keybinding: &Bind) {
        self.speed += match (
            is_key_down(keybinding.get_up()),
            is_key_down(keybinding.get_down()),
            is_key_down(keybinding.get_left()),
            is_key_down(keybinding.get_right()),
        ) {
            (true, false, true, false) => Vec2::new(-1.0, -1.0) * Self::SPACE_SHIP_ACCELERATION,
            (true, false, false, true) => Vec2::new(1.0, -1.0) * Self::SPACE_SHIP_ACCELERATION,
            (false, true, true, false) => Vec2::new(-1.0, 1.0) * Self::SPACE_SHIP_ACCELERATION,
            (false, true, false, true) => Vec2::new(1.0, 1.0) * Self::SPACE_SHIP_ACCELERATION,
            (true, false, false, false) => Vec2::new(0.0, -1.0) * Self::SPACE_SHIP_ACCELERATION,
            (false, true, false, false) => Vec2::new(0.0, 1.0) * Self::SPACE_SHIP_ACCELERATION,
            (false, false, true, false) => Vec2::new(-1.0, 0.0) * Self::SPACE_SHIP_ACCELERATION,
            (false, false, false, true) => Vec2::new(1.0, 0.0) * Self::SPACE_SHIP_ACCELERATION,
            _ => Vec2::new(0.0, 0.0),
        };
        self.speed *= Self::SPACE_SHIP_DECELERATION;

        self.position += self.speed;
        self.position = Spaceship::bound_pos(self.position);
    }
    /// Dessine le vaisseau spatial en mode classique.
    ///
    /// # Arguments
    ///
    /// - `texture`: La texture utilisée pour représenter le vaisseau.
    /// - `time`: Le temps actuel (en secondes).
    pub fn draw_spaceship_classic(&self, texture: &Texture2D, time: f64) {
        if time - self.last_collision_time < 2.0 && has_even_decimal_part(time) {
            return;
        }
        if self.shield != 1 {
            draw_circle(
                self.get_position().x,
                self.get_position().y,
                self.get_size() + 20.0,
                Color::new(0.0, 1.0, 0.0, 0.5),
            );
        }
        let thickness = 2.0;
        let rota = self.get_rotation();
        draw_texture_ex(
            texture,
            self.get_position().x - (self.get_size() * thickness / 2.0),
            self.get_position().y - (self.get_size() * thickness / 2.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    self.get_size() * thickness,
                    self.get_size() * thickness,
                )),
                rotation: rota,
                ..Default::default()
            },
        );
    }

    /// Dessine le vaisseau spatial en mode moderne.
    ///
    /// # Arguments
    ///
    /// - `texture`: La texture utilisée pour représenter le vaisseau.
    /// - `time`: Le temps actuel (en secondes).
    pub fn draw_spaceship_modern(&self, texture: &Texture2D, time: f64) {
        if time - self.last_collision_time < 2.0 && has_even_decimal_part(time) {
            return;
        }
        if self.shield != 1 {
            draw_circle(
                self.get_position().x,
                self.get_position().y,
                self.get_size() + 20.0,
                Color::new(0.0, 1.0, 0.0, 0.5),
            );
        }
        // To make the image thicker
        let thickness = 2.0;
        let rota = self.angle_from_mouse();
        draw_texture_ex(
            texture,
            self.get_position().x - (self.get_size() * thickness / 2.0),
            self.get_position().y - (self.get_size() * thickness / 2.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    self.get_size() * thickness,
                    self.get_size() * thickness,
                )),
                rotation: rota.y.atan2(rota.x) + std::f32::consts::FRAC_PI_2,
                ..Default::default()
            },
        );
    }
}

impl_stellar_object!(Spaceship);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asteroid::AsteroidShape;

    #[test]
    fn test_spaceship_new() {
        let spaceship = Spaceship::new();
        assert_eq!(spaceship.get_position(), Vec2::new(220.0, 330.0));
        assert_eq!(spaceship.get_size(), Spaceship::SPACE_SHIP_SIZE);
        assert_eq!(spaceship.get_rotation(), 0.0);
        assert_eq!(spaceship.shield, 3);
        assert_eq!(spaceship.last_collision_time, -2.0);
        assert_eq!(spaceship.score, 0);
    }

    #[test]
    fn test_spaceship_collision_asteroid() {
        let spaceship_test: Spaceship = Spaceship {
            // Attributs fictifs pour le test
            position: Vec2::new(200.0, 300.0),
            speed: Vec2::new(0.0, 0.0),
            size: Spaceship::SPACE_SHIP_SIZE,
            rotation: 0.0,
            shield: 3,
            last_collision_time: 0.0,
            score: 0,
        };

        let asteroid_test = Asteroid {
            // Attributs fictifs pour le test
            position: Vec2::new(220.0, 330.0),
            speed: Vec2::new(0.0, 0.0),
            shape: AsteroidShape::Large,
            size: 20.0,
            rotation: 0.0,
            birth_time: 0.0,
        };
        assert_eq!(spaceship_test.collision(&asteroid_test), true);
    }

    #[test]
    fn test_spaceship_collision_blackhole() {
        let spaceship_test: Spaceship = Spaceship {
            // Attributs fictifs pour le test
            position: Vec2::new(200.0, 300.0),
            speed: Vec2::new(0.0, 0.0),
            size: Spaceship::SPACE_SHIP_SIZE,
            rotation: 0.0,
            shield: 3,
            last_collision_time: 0.0,
            score: 0,
        };

        let blackhole_test = BlackHole {
            position: Vec2::new(220.0, 330.0),
            size: 20.0,
            rotation: 0.0,
        };
        assert_eq!(spaceship_test.collision_blackhole(&blackhole_test), true);
    }
}
