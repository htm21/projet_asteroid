use crate::asteroid::{Asteroid, AsteroidShape};
use crate::Spaceship;

// Getter de la valeur du score du vaisseau spatial.
//
// # Arguments
// - `spaceship` - Référence du vaisseau spatial.
//
// # Returns
// - `u32` Valeur du score du vaisseau spatial.
//
pub fn get_score(spaceship: &Spaceship) -> u32 {
    spaceship.score
}

// Ajoute un score au vaisseau spatial.
//
// # Arguments
// - `spaceship` - Référence du vaisseau spatial.
// - `score` - Score à ajouter.
//
pub fn add_score(spaceship: &mut Spaceship, score: u32) {
    spaceship.score += score;
}

// Augmente le score du vaisseau spatial en fonction de la taille de l'astéroid détruit.
//
// # Arguments
// - `asteroid` - Référence de l'astéroid détruit.
//
// # Returns
// - `u32` Score à ajouter.
//
pub fn increase_score(asteroid: &Asteroid) -> u32 {
    let mut score = 0;
    match asteroid.get_shape() {
        AsteroidShape::Large => score += 25,
        AsteroidShape::Medium => score += 50,
        AsteroidShape::Small => score += 100,
    }
    score
}
