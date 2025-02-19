use crate::sound::Sound;
use macroquad::prelude::*;

use crate::{
    asteroid::Asteroid,
    impl_stellar_object,
    score::{add_score, increase_score},
    spaceship::Spaceship,
    stellarobject::StellarObject,
};

/// Structure d'un missile.
pub struct Missile {
    pub position: Vec2,
    pub speed: Vec2,
    pub size: f32,
    pub thickness: f32,
}

/// Implémentation du missile.
impl Missile {
    /// Créer un nouveau missile.
    ///
    /// # Arguments
    /// - `position` La position du missile.
    /// - `speed` La vitesse du missile.
    /// - `size` La taille du missile.
    /// - `thickness` L'épaisseur du missile (ligne).
    ///
    pub fn new(position: Vec2, speed: Vec2, size: f32, thickness: f32) -> Self {
        Self {
            position,
            speed,
            size,
            thickness,
        }
    }

    /// Getter de la position de fin du missile.
    ///
    /// # Returns
    /// - `Vec2` Le vecteur position de fin du missile.
    ///
    fn get_end_position(&self) -> Vec2 {
        self.position + self.speed * self.size
    }

    /// Affichage du missile.
    /// Le missile est dessiné sous forme de ligne.
    pub fn draw_missile(&self) {
        let pos = self.get_end_position();
        draw_line(
            self.position.x,
            self.position.y,
            pos[0],
            pos[1],
            self.thickness,
            WHITE,
        );
    }

    /// Mise à jour de la position du missile.
    pub fn move_missile(&mut self) {
        self.position += self.speed;
    }

    /// Vérifie si le missile est en collision avec un astéroid.
    ///
    /// # Arguments
    /// - `asteroid` L'astéroid à vérifier.
    ///
    /// # Returns
    /// - `true` si le missile est en collision avec l'astéroid.
    pub fn is_collide_asteroid(&self, asteroid: &Asteroid) -> bool {
        let a: Vec2 = self.position;
        let b: Vec2 = self.get_end_position();
        let c: Vec2 = asteroid.get_position();

        // Vecteurs ab et ac
        let ab = (b.x - a.x, b.y - a.y);
        let ac = (c.x - a.x, c.y - a.y);

        // Scalar product and projection
        let dot_product = ab.0 * ac.0 + ab.1 * ac.1;
        let length_squared = ab.0 * ab.0 + ab.1 * ab.1;
        let proj = (dot_product / length_squared).clamp(0.0, 1.0);

        // Closest point
        let closest_point = vec2(a.x + proj * ab.0, a.y + proj * ab.1);

        // Distance between the closest point and the center of the circle
        let distance = ((closest_point.x - c.x).powi(2) + (closest_point.y - c.y).powi(2)).sqrt();

        // Check if the distance is less than the radius of the circle
        distance <= asteroid.get_size()
    }

    /// Gestions des tirs de missiles en mode 'classic'.
    ///
    /// # Arguments
    /// - `key` La touche 'Space' (espace).
    /// - `last_shoot` Le temps du dernier tir.
    /// - `time` Le temps actuel.
    ///
    /// # Returns
    /// - `true` si le vaisseau tire un missile
    ///
    pub fn do_i_shoot_classic(key: KeyCode, last_shoot: f64, time: f64) -> bool {
        is_key_down(key) && time - last_shoot > 0.2
    }

    /// Gestions des tirs de missiles en mode 'modern'.
    /// Les tirs sont effectués avec les clics de la souris.
    ///
    /// # Arguments
    /// - `last_shoot` Le temps du dernier tir.
    /// - `time` Le temps actuel.
    ///
    pub fn do_i_shoot_modern(last_shoot: f64, time: f64) -> bool {
        (is_mouse_button_down(MouseButton::Left)
            || is_mouse_button_down(MouseButton::Middle)
            || is_mouse_button_down(MouseButton::Right))
            && time - last_shoot > 0.2
    }

    /// Créer un nouveau missile en mode 'classic'.
    /// A pour origine l'orientation du vaisseau spatial (gérée par les touches directionnelles).
    ///
    /// # Arguments
    /// - `spaceship` Le vaisseau spatial.
    /// - `factor` Facteur de vitesse du missile.
    ///
    pub fn new_missile_classic(spaceship: &Spaceship, factor: f32) -> Self {
        let source = spaceship.get_position();
        let speed: Vec2 = Vec2::new(
            spaceship.get_rotation().sin(),
            -spaceship.get_rotation().cos(),
        ) * factor;

        if speed == Vec2::new(0.0, 0.0) {
            let rotation_vec = Vec2::new(
                spaceship.get_rotation().sin(),
                -spaceship.get_rotation().cos(),
            );
            Missile::new(source, rotation_vec * factor, 5.0, 2.0)
        } else {
            Missile::new(source, speed, 5.0, 2.0)
        }
    }

    /// Créer un nouveau missile en mode 'modern'.
    /// A pour origine l'orientation du vaisseau spatial (gérée par le curseur de la souris).
    ///
    /// # Arguments
    /// - `spaceship` Le vaisseau spatial.
    /// - `factor` Facteur de vitesse du missile.
    ///
    pub fn new_missile_modern(spaceship: &Spaceship, factor: f32) -> Self {
        let source = spaceship.get_position();
        let speed = spaceship.angle_from_mouse() * factor;
        Self::new(source, speed, 5.0, 2.0)
    }

    /// Gestion des collisions entre les missiles et les astéroids.
    /// Permet de vérifier si un missile est entré en collision avec un astéroid.
    /// Si c'est le cas, le missile est supprimé, l'astéroid est scindé en deux (si sa taille le lui permet) et le score est augmenté.
    ///
    /// # Arguments
    /// - `missiles` Les missiles.
    /// - `asteroids` Les astéroids.
    /// - `spaceship` Le vaisseau spatial.
    /// - `sound` Les sons du jeu.
    ///
    pub fn what_collide_missile(
        missiles: &mut Vec<Missile>,
        asteroids: &mut Vec<Asteroid>,
        spaceship: &mut Spaceship,
        sound: &Sound,
    ) {
        let mut i = 0;

        while i < missiles.len() {
            let mut j = 0;
            while j < asteroids.len() {
                if missiles[i].is_collide_asteroid(&asteroids[j]) {
                    sound.play_sound_effect("boom");
                    let new_asteroids = asteroids[j].split();

                    add_score(spaceship, increase_score(&asteroids[j]));

                    asteroids.swap_remove(j);
                    missiles.swap_remove(i);

                    asteroids.extend(new_asteroids);

                    i = i.saturating_sub(1);
                    break;
                }
                j += 1;
            }
            i += 1;
        }
    }

    /// Suppression des missiles dès lorsqu'ils quittent l'écran.
    ///
    /// # Arguments
    /// - `missiles` Les missiles.
    ///
    pub fn clear_missiles(missiles: &mut Vec<Self>) {
        let mut i = 0;
        while i < missiles.len() {
            let position = missiles[i].get_position();
            if position.x < 0.0
                || position.x > screen_width()
                || position.y < 0.0
                || position.y > screen_height()
            {
                missiles.swap_remove(i);
            }
            i += 1;
        }
    }
}
impl_stellar_object!(Missile);

#[cfg(test)]
mod tests {
    use crate::asteroid::AsteroidShape;

    use super::*;

    #[test]
    fn test_missile_new() {
        let missile: Missile = Missile::new(Vec2::new(100.0, 100.0), Vec2::new(1.0, 1.0), 5.0, 2.0);
        assert_eq!(missile.get_position(), Vec2::new(100.0, 100.0));
    }

    #[test]
    fn test_collide() {
        let missile: Missile = Missile::new(Vec2::new(100.0, 100.0), Vec2::new(1.0, 1.0), 5.0, 2.0);
        let asteroid: Asteroid = Asteroid::new(
            Vec2::new(100.0, 100.0),
            Vec2::new(1.0, 1.0),
            AsteroidShape::Large,
            2.0,
        );
        assert_eq!(missile.is_collide_asteroid(&asteroid), true);
    }

    #[test]
    fn test_get_end_position() {
        let missile: Missile = Missile::new(Vec2::new(100.0, 100.0), Vec2::new(1.0, 1.0), 5.0, 2.0);
        assert_eq!(missile.get_end_position(), Vec2::new(105.0, 105.0));
    }
}
