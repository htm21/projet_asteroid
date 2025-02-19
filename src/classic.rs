use super::draw_background;
use super::draw_time;
use super::utils::sigmoid_range_secs;
use crate::bind::Bind;
use crate::blackhole::BlackHole;
use crate::gui::draw_asteroid_number;
use crate::gui::draw_score;
use crate::gui::draw_shield;
use crate::sound::Sound;
use crate::utils::sigmoid_speed;
use crate::{asteroid::Asteroid, missile::Missile, spaceship::Spaceship};
use macroquad::prelude::*;

/// Affichage du jeu pour le mode classique.
///     
/// # Arguments
/// - `skins` Les textures des objets du jeu.
/// - `asteroids` Les astéroids.
/// - `blackholes` Les trous noirs.
/// - `spaceship` Le vaisseau spatial.
/// - `time` Le temps de jeu.
/// - `missiles` Les missiles tirés par le vaisseau spatial.
///
pub fn draw_classic(
    skins: [&Texture2D; 4],
    asteroids: &Vec<Asteroid>,
    blackholes: &Vec<BlackHole>,
    spaceship: &Spaceship,
    time: f64,
    missiles: &Vec<Missile>,
) {
    let asteroid_texture = skins[0];
    let blackhole_texture = skins[1];
    let background_texture = skins[2];
    let spaceship_texture = skins[3];

    let time_str = format!("{:.2}", time);
    draw_background(background_texture);

    for asteroid in asteroids {
        asteroid.draw_asteroid(asteroid_texture);
    }
    for blackhole in blackholes {
        blackhole.draw_blackhole(blackhole_texture);
    }
    spaceship.draw_spaceship_classic(spaceship_texture, time);
    draw_time(&time_str);
    draw_shield(spaceship.shield);
    draw_score(spaceship.score);

    draw_asteroid_number(asteroids);

    for missile in missiles {
        missile.draw_missile();
    }
}

/// Mise à jour du jeu pour le mode 'classic'.
/// Permet le fonctionnement du jeu en mode 'classic', avec la mise à jour des positions des objets et des collisions.
///
/// # Arguments
/// - `asteroids` Les astéroids.
/// - `blackholes` Les trous noirs.
/// - `last_add_asteroid` Le temps de la dernière apparition d'astéroid (pour contrôler le temps entre chaque apparition).
/// - `spaceship` Le vaisseau spatial.
/// - `time` Le temps de jeu.
/// - `last_shoot` Le temps du dernier tir du vaisseau spatial.
/// - `missiles` Les missiles tirés par le vaisseau spatial.
/// - `sound` Les sons du jeu.
/// - `keybinding` Les touches du clavier utilisées pour contrôler le vaisseau spatial (choisi par le joueur).
///
#[allow(clippy::too_many_arguments)]
pub async fn update_classic(
    asteroids: &mut Vec<Asteroid>,
    blackholes: &mut Vec<BlackHole>,
    last_add_asteroid: &mut f64,
    spaceship: &mut Spaceship,
    time: f64,
    last_shoot: &mut f64,
    missiles: &mut Vec<Missile>,
    sound: &Sound,
    keybinding: &Bind,
) -> bool {
    if time < 45.0
        && Asteroid::push_or_dont(
            asteroids,
            last_add_asteroid,
            time,
            sigmoid_range_secs(time, 22.5),
        )
    {
        *last_add_asteroid = time;
    }

    for asteroid in &mut *asteroids {
        asteroid.move_asteroid(true, sigmoid_speed(time, "classic"));
    }

    spaceship.update_position_classic(keybinding);

    if spaceship.spaceship_collision(&mut *asteroids).await && spaceship.check_shield(time, sound) {
        return true;
    }

    if spaceship
        .spaceship_blackhole_collision(&mut *blackholes)
        .await
    {
        return true;
    }

    if Missile::do_i_shoot_classic(KeyCode::Space, *last_shoot, time) {
        missiles.push(Missile::new_missile_classic(spaceship, 5.0));
        *last_shoot = time;
        sound.play_sound_effect("shoot");
    }
    for missile in &mut *missiles {
        missile.move_missile();
    }
    Missile::clear_missiles(&mut *missiles);
    Missile::what_collide_missile(&mut *missiles, &mut *asteroids, spaceship, sound);
    false
}
