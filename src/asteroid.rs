//! # Asteroid Module
//!
//! Ce module gère les astéroïdes dans un jeu spatial. Il inclut la structure `Asteroid`,
//! des énumérations pour les différentes formes d'astéroïdes, ainsi que des fonctions
//! pour créer, manipuler, et détecter les collisions entre eux.
//!
//! ## Contenu
//! - Structure `Asteroid`
//! - Enumération `AsteroidShape`
//! - Génération aléatoire d'astéroïdes
//! - Gestion des collisions et des déplacements
//!
//! ## Exemple
//!
//! ```rust
//! use my_game::asteroid::{Asteroid, AsteroidShape};
//! use macroquad::prelude::*;
//!
//!
//!     let position = Vec2::new(100.0, 100.0);
//!     let speed = Vec2::new(1.0, 0.5);
//!     let asteroid = Asteroid::new(position, speed, AsteroidShape::Large, 70.0);
//!
//!     println!("Asteroid créé : {:?}", asteroid);
//!
//! ```
use crate::blackhole::BlackHole;
use crate::impl_stellar_object;
use crate::stellarobject::StellarObject;
use ::rand::distributions::{Distribution, Standard};
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::f32::consts::PI;

/// Représente un astéroïde dans l'espace.
pub struct Asteroid {
    /// Position de l'astéroïde dans l'espace.
    pub position: Vec2,
    /// Vitesse actuelle de l'astéroïde.
    pub speed: Vec2,
    /// Type de l'astéroïde (`Small`, `Medium`, `Large`).
    pub shape: AsteroidShape,
    /// Taille de l'astéroïde.
    pub size: f32,
    /// Rotation de l'astéroïde.
    pub rotation: f32,
    /// Temps de création de l'astéroïde.
    pub birth_time: f64,
}

/// Définit les formes possibles pour les astéroïdes.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AsteroidShape {
    /// Petit astéroïde.
    Small,
    /// Astéroïde de taille moyenne.
    Medium,
    /// Grand astéroïde.
    Large,
}

// Implémentation de la distribution des types d'asteroids.
impl Distribution<AsteroidShape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AsteroidShape {
        match rng.gen_range(0..=2) {
            0 => AsteroidShape::Small,
            1 => AsteroidShape::Medium,
            _ => AsteroidShape::Large,
        }
    }
}

impl Asteroid {
    /// Taille d'un petit astéroïde.
    pub const ASTEROID_SMALL_SIZE: f32 = 30.0;
    /// Taille d'un astéroïde de taille moyenne.
    pub const ASTEROID_MEDIUM_SIZE: f32 = 50.0;
    /// Taille d'un grand astéroïde.
    pub const ASTEROID_LARGE_SIZE: f32 = 70.0;

    /// Crée un nouvel astéroïde.
    ///
    /// # Arguments
    /// - `position`: Position initiale de l'astéroïde.
    /// - `speed`: Vitesse initiale de l'astéroïde.
    /// - `shape`: Type de l'astéroïde (`AsteroidShape`).
    /// - `size`: Taille de l'astéroïde.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// let asteroid = Asteroid::new(
    ///     Vec2::new(100.0, 100.0),
    ///     Vec2::new(1.0, 0.0),
    ///     AsteroidShape::Large,
    ///     70.0
    /// );
    /// ```
    pub fn new(position: Vec2, speed: Vec2, shape: AsteroidShape, size: f32) -> Self {
        let mut rng = thread_rng();

        let rotation = if rng.gen_bool(0.5) { 0.01 } else { -0.01 };

        Self {
            position,
            speed,
            shape,
            size,
            birth_time: if cfg!(test) { 0.0 } else { get_time() }, // Pour éviter les problèmes de 'get_time()' dans les tests. (macroquad n'est pas initialisé dans cette section)
            rotation,
        }
    }

    /// Crée un astéroïde avec des paramètres aléatoires.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// let random_asteroid = Asteroid::new_random();
    /// println!("Astéroïde aléatoire : {:?}", random_asteroid);
    /// ```
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let shape: AsteroidShape = rng.gen();
        let rotation = if rng.gen_bool(0.5) { 0.01 } else { -0.01 };
        let size = match shape {
            AsteroidShape::Large => {
                rng.gen_range(Self::ASTEROID_MEDIUM_SIZE..=Self::ASTEROID_LARGE_SIZE)
            }
            AsteroidShape::Medium => {
                rng.gen_range(Self::ASTEROID_SMALL_SIZE..=Self::ASTEROID_MEDIUM_SIZE)
            }
            AsteroidShape::Small => rng.gen_range(10.0..=Self::ASTEROID_SMALL_SIZE),
        };

        Self {
            position: Self::new_random_alea_pos(size),
            speed: Self::new_random_alea_speed(),
            shape,
            size,
            birth_time: get_time(),
            rotation,
        }
    }

    /// Getter du 'type' de l'asteroid.
    pub fn get_shape(&self) -> AsteroidShape {
        self.shape
    }

    /// Getter du moment de création de l'asteroid.
    pub fn get_birth_time(&self) -> f64 {
        self.birth_time
    }
    /// Getter de l'orientatin de l'asteroid.
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    // Setter de la rotation de l'asteroid.
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    /// Casse l'astéroid en deux autres plus petits.
    ///
    /// # Returns
    /// - `Vec<Asteroid>` Liste des nouveaux astéroids.
    ///   
    /// # Examples
    ///     
    /// ```
    /// let asteroid = Asteroid::new(Vec2::new(100.0, 100.0), Vec2::new(0.0, 0.0), AsteroidShape::Large, 70.0);
    /// let new_asteroids = asteroid.split();
    /// assert_eq!(new_asteroids.len(), 2);
    /// ```
    ///
    pub fn split(&self) -> Vec<Asteroid> {
        let mut rng = thread_rng();
        match self.get_shape() {
            AsteroidShape::Large => {
                let mut asteroids = Vec::new();
                for _ in 0..2 {
                    asteroids.push(Asteroid::new(
                        self.position,
                        Self::new_random_alea_speed(),
                        AsteroidShape::Medium,
                        rng.gen_range(Self::ASTEROID_SMALL_SIZE..=Self::ASTEROID_MEDIUM_SIZE),
                    ));
                }
                asteroids
            }
            AsteroidShape::Medium => {
                let mut asteroids = Vec::new();
                for _ in 0..2 {
                    asteroids.push(Asteroid::new(
                        self.position,
                        Self::new_random_alea_speed(),
                        AsteroidShape::Small,
                        rng.gen_range(20.0..=Self::ASTEROID_SMALL_SIZE),
                    ));
                }
                asteroids
            }
            AsteroidShape::Small => Vec::new(),
        }
    }

    /// Génère une position aléatoire près de l'un des bords.
    ///
    /// # Arguments
    /// - `size` - Taille de l'astéroid.
    ///
    /// # Returns
    /// - `Vec2` Position aléatoire.
    ///
    fn new_random_alea_pos(size: f32) -> Vec2 {
        let mut rng = thread_rng();

        let nearpos: f32 = rng.gen_range(size / 2.0..=size);
        let nearside = rng.gen_range(1..=4); // 1 = top, 2 = right, 3 = down, 4 = left
        let xpos: f32 = match nearside {
            2 => screen_width() - nearpos,
            4 => nearpos,
            _ => rng.gen_range(0.0..=screen_width()),
        };
        let ypos: f32 = match nearside {
            1 => nearpos,
            3 => screen_height() - nearpos,
            _ => rng.gen_range(0.0..=screen_height()),
        };
        vec2(xpos, ypos)
    }

    /// Génère une vitesse aléatoire.
    ///
    /// # Returns
    /// - `Vec2` Vitesse aléatoire.
    ///
    fn new_random_alea_speed() -> Vec2 {
        let mut rng = thread_rng();

        let angle: f32 = rng.gen_range(0.0..=(2.0 * PI));
        Vec2::from_angle(angle)
    }

    /// Vérifie si l'astéroïde est en collision avec un autre astéroïde.
    ///
    /// # Arguments
    /// - `other`: Référence à un autre astéroïde.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// let a1 = Asteroid::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), AsteroidShape::Small, 30.0);
    /// let a2 = Asteroid::new(Vec2::new(10.0, 0.0), Vec2::new(0.0, 1.0), AsteroidShape::Small, 30.0);
    /// let is_colliding = a1.is_collide_round(&a2);
    /// println!("Collision détectée : {}", is_colliding);
    /// ```
    pub fn is_collide_round(&self, other: &Self) -> bool {
        let distance = Vec2::distance(self.get_position(), other.get_position());
        let sum_radius = self.get_size() + other.get_size();
        distance < sum_radius
    }

    /// Met à jour la vitesse et l'orientation de l'asteroid (vitesse de plus en plus élevée).
    ///
    /// # Arguments
    /// - `rotation` L'orientation de l'asteroid.
    /// - `factor` Facteur de vitesse (qui augmente).
    ///
    /// # Returns
    /// - `Vec2` Nouvelle position de l'asteroid.
    ///
    pub fn move_asteroid(&mut self, rotation: bool, factor: f64) -> Vec2 {
        self.set_position(self.position + self.get_speed() * factor as f32);
        self.position = Self::bound_pos(self.get_position());
        if !rotation {
            return self.position;
        }
        if self.rotation > 0.0 {
            self.set_rotation(self.rotation + 0.01);
        } else {
            self.set_rotation(self.rotation - 0.01);
        }
        self.position * factor as f32
    }

    /// Dessine l'astéroid à partir de sa texture (son skin).
    ///     
    /// # Arguments
    /// - `texture` La texture de l'astéroid.
    ///
    pub fn draw_asteroid(&self, texture: &Texture2D) {
        draw_texture_ex(
            texture,
            self.get_position().x - (self.get_size() / 2.0),
            self.get_position().y - (self.get_size() / 2.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.get_size() * 2.0, self.get_size() * 2.0)),
                rotation: self.get_rotation(),
                ..Default::default()
            },
        );
    }
    /// Décide si un nouvel astéroïde doit être ajouté à la liste en fonction du temps écoulé
    /// depuis le dernier ajout.
    ///
    /// # Arguments
    ///
    /// - `asteroids`: Une référence mutable à un vecteur contenant les astéroïdes existants.
    /// - `last_add`: Une référence mutable à un timestamp (en secondes) représentant le moment
    ///   où le dernier astéroïde a été ajouté.
    /// - `time`: Le timestamp actuel (en secondes).
    /// - `range_secs`: Le délai minimum (en secondes) entre deux ajouts successifs d'astéroïdes.
    ///
    /// # Returns
    ///
    /// - `true` si un nouvel astéroïde a été ajouté.
    /// - `false` si aucun astéroïde n'a été ajouté (le délai n'était pas respecté).
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use my_game::asteroid::Asteroid;
    ///
    /// let mut asteroids = Vec::new();
    /// let mut last_add = 0.0;
    /// let current_time = 10.0;
    /// let range_secs = 5.0;
    ///
    /// let result = Asteroid::push_or_dont(&mut asteroids, &mut last_add, current_time, range_secs);
    ///
    /// assert!(result); // Un astéroïde a été ajouté car suffisamment de temps s'est écoulé.
    /// assert_eq!(asteroids.len(), 1);
    /// ```
    ///
    /// # Notes
    ///
    /// Cette fonction est utile pour réguler la fréquence d'apparition des astéroïdes
    /// dans un environnement de jeu où ils doivent apparaître de manière contrôlée.
    pub fn push_or_dont(
        asteroids: &mut Vec<Asteroid>,
        last_add: &mut f64,
        time: f64,
        range_secs: f64,
    ) -> bool {
        if time - range_secs > *last_add {
            asteroids.push(Self::new_random());
            *last_add = time;
            return true;
        }
        false
    }

    /// Vérifie si l'astéroid est entré en collision avec un autre astéroid ou un trou noir.
    /// Si c'est le cas, les deux astéroids fusionnent pour former un trou noir (événement aléatoire du mode 'modern').
    ///  
    /// # Arguments
    /// - `astéroid` Liste des astéroids.
    /// - `blackholes` Liste des trous noirs.
    ///
    pub fn what_collide_asteroids(asteroids: &mut Vec<Asteroid>, blackholes: &mut Vec<BlackHole>) {
        let mut i = 0;
        let current_time = if cfg!(test) { 10.0 } else { get_time() }; // Pour éviter les problèmes de 'get_time()' dans les tests. (macroquad n'est pas initialisé dans cette section)
        let mut to_remove = Vec::new();
        let mut new_asteroids = Vec::new();

        while i < asteroids.len() {
            let mut j = i + 1;
            while j < asteroids.len() {
                if asteroids[i].is_collide_round(&asteroids[j])
                    && current_time - asteroids[i].get_birth_time() > 1.0
                    && current_time - asteroids[j].get_birth_time() > 1.0
                    && if cfg!(test) {
                        true
                    } else {
                        thread_rng().gen_range(0..15) == 0
                    }
                // Pour assurer la condition de l'événement dans le test.
                {
                    blackholes.push(BlackHole::new(asteroids[i].get_position()));

                    to_remove.push(i);
                    to_remove.push(j);

                    break;
                } else if asteroids[i].is_collide_round(&asteroids[j])
                    && current_time - asteroids[i].get_birth_time() > 1.0
                    && current_time - asteroids[j].get_birth_time() > 1.0
                {
                    new_asteroids.extend(asteroids[i].split());
                    new_asteroids.extend(asteroids[j].split());

                    to_remove.push(i);
                    to_remove.push(j);

                    break;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }

        to_remove.sort_unstable();
        to_remove.dedup();
        for &index in to_remove.iter().rev() {
            asteroids.swap_remove(index);
        }

        asteroids.extend(new_asteroids);
    }
}

impl_stellar_object!(Asteroid);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asteroid_new() {
        let asteroid: Asteroid = Asteroid::new(
            Vec2::new(100.0, 100.0),
            Vec2::new(1.0, 0.5),
            AsteroidShape::Large,
            70.0,
        );
        assert_eq!(asteroid.get_position(), Vec2::new(100.0, 100.0));
    }

    #[test]
    fn test_asteroid_split() {
        let asteroid_test = Asteroid::new(
            Vec2::new(100.0, 100.0),
            Vec2::new(0.0, 0.0),
            AsteroidShape::Large,
            70.0,
        );
        let asteroids_test = asteroid_test.split();
        assert_eq!(asteroids_test.len(), 2);
    }

    #[test]
    fn test_asteroid_is_collide_round() {
        let asteroid_test_1 = Asteroid::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            AsteroidShape::Small,
            30.0,
        );
        let asteroid_test_2 = Asteroid::new(
            Vec2::new(10.0, 20.0),
            Vec2::new(0.0, 1.0),
            AsteroidShape::Small,
            30.0,
        );
        assert_eq!(asteroid_test_1.is_collide_round(&asteroid_test_2), true);
    }

    #[test]
    fn test_asteroid_move_asteroid() {
        let mut asteroid_test = Asteroid::new(
            Vec2::new(100.0, 100.0),
            Vec2::new(1.0, 0.0),
            AsteroidShape::Large,
            70.0,
        );
        let new_rotation_value = if asteroid_test.get_rotation() > 0.0 {
            asteroid_test.get_rotation() + 0.01
        } else {
            asteroid_test.get_rotation() - 0.01
        };
        asteroid_test.move_asteroid(true, 1.0);
        assert_eq!(asteroid_test.get_rotation(), new_rotation_value);
    }

    #[test]
    fn test_blackhole_creation() {
        let mut asteroids_test = Vec::new();
        let asteroid1_test = Asteroid::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            AsteroidShape::Small,
            70.0,
        );
        let asteroid2_test = Asteroid::new(
            Vec2::new(10.0, 20.0),
            Vec2::new(1.0, 0.0),
            AsteroidShape::Small,
            70.0,
        );
        asteroids_test.push(asteroid1_test);
        asteroids_test.push(asteroid2_test);

        let mut blackholes_test = Vec::new();

        Asteroid::what_collide_asteroids(&mut asteroids_test, &mut blackholes_test);
        assert_eq!(blackholes_test.len(), 1);
    }
}
