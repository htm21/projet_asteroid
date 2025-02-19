// Modules
mod asteroid;
mod bind;
mod blackhole;
mod classic;
mod gui;
mod import;
mod menu;
mod missile;
mod modern;
mod option;
mod score;
mod skins;
mod sound;
mod spaceship;
mod stellarobject;
mod utils;

// Dépendances externes
use crate::blackhole::BlackHole;
use asteroid::Asteroid;
use bind::Bind;
use classic::{draw_classic, update_classic};
use gui::{draw_time, game_over, pause};
use macroquad::prelude::*;
use menu::menu;
use modern::{draw_modern, update_modern};
use skins::skins;
use sound::Sound;
use spaceship::Spaceship;

/// Dessine l'arrière-plan du jeu.
///
/// # Paramètres
/// - `background_texture`: Texture2D - La texture de l'arrière-plan à afficher.
fn draw_background(background_texture: &Texture2D) {
    draw_texture_ex(
        background_texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())), // Adapter à la taille de l'écran
            ..Default::default()
        },
    );
}

/// Boucle principale de jeu pour les modes "classic" et "modern".
///
/// # Paramètres
/// - `mode`: &str - Mode de jeu sélectionné ("classic" ou "modern").
/// - `skins`: [&str; 4] - Chemins des fichiers pour les skins (astéroïdes, trous noirs, arrière-plan, vaisseau).
/// - `start_time`: f64 - Temps de démarrage du jeu.
/// - `sound`: &mut Sound - Gestionnaire des sons.
/// - `bind`: &mut Bind - Gestionnaire des commandes utilisateur.
async fn play(mode: &str, skins: [&str; 4], start_time: f64, sound: &mut Sound, bind: &mut Bind) {
    // Chargement des textures
    let asteroid_texture = load_texture(skins[0]).await.unwrap();
    asteroid_texture.set_filter(FilterMode::Linear);
    let blackhole_texture = load_texture(skins[1]).await.unwrap();
    blackhole_texture.set_filter(FilterMode::Linear);
    let background_texture = load_texture(skins[2]).await.unwrap();
    background_texture.set_filter(FilterMode::Nearest);
    let spaceship_texture = load_texture(skins[3]).await.unwrap();
    spaceship_texture.set_filter(FilterMode::Linear);

    // Initialisation des entités
    let mut blackholes: Vec<BlackHole> = Vec::new();
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut last_add_asteroid: f64 = 0.0;
    let mut time: f64;
    let mut total_paused_time: f64 = 0.0;
    let mut spaceship = Spaceship::new();
    let mut missiles: Vec<missile::Missile> = Vec::new();
    let mut last_shoot: f64 = 0.0;

    // Boucle principale selon le mode
    match mode {
        "classic" => {
            loop {
                time = get_time() - start_time - total_paused_time;

                // Gérer la pause
                total_paused_time +=
                    pause(&background_texture, &asteroid_texture, sound, bind).await;

                // Quitter si Backspace est pressé
                if is_key_down(KeyCode::Backspace) {
                    break;
                }

                // Mettre à jour les entités et vérifier les conditions de fin
                if update_classic(
                    &mut asteroids,
                    &mut blackholes,
                    &mut last_add_asteroid,
                    &mut spaceship,
                    time,
                    &mut last_shoot,
                    &mut missiles,
                    sound,
                    bind,
                )
                .await
                {
                    if game_over(&spaceship, sound, false).await {
                        break;
                    } else {
                        std::process::exit(0);
                    }
                }

                // Dessiner les éléments
                draw_classic(
                    [
                        &asteroid_texture,
                        &blackhole_texture,
                        &background_texture,
                        &spaceship_texture,
                    ],
                    &asteroids,
                    &blackholes,
                    &spaceship,
                    time,
                    &missiles,
                );
                if asteroids.is_empty() && time > 10.0 {
                    if game_over(&spaceship, sound, true).await {
                        break;
                    } else {
                        std::process::exit(0);
                    }
                }
                next_frame().await;
            }
        }
        "modern" => loop {
            time = get_time() - start_time - total_paused_time;

            total_paused_time += pause(&background_texture, &asteroid_texture, sound, bind).await;

            if is_key_down(KeyCode::Backspace) {
                break;
            }

            if update_modern(
                &mut asteroids,
                &mut blackholes,
                &mut last_add_asteroid,
                &mut spaceship,
                time,
                &mut last_shoot,
                &mut missiles,
                sound,
                bind,
            )
            .await
            {
                if game_over(&spaceship, sound, false).await {
                    break;
                } else {
                    std::process::exit(0);
                }
            }

            draw_modern(
                [
                    &asteroid_texture,
                    &blackhole_texture,
                    &background_texture,
                    &spaceship_texture,
                ],
                &asteroids,
                &blackholes,
                &spaceship,
                time,
                &missiles,
            );
            if asteroids.is_empty() && time > 45.0 {
                if game_over(&spaceship, sound, true).await {
                    break;
                } else {
                    std::process::exit(0);
                }
            }
            next_frame().await;
        },
        _ => {
            eprintln!("Unknown mode: {}", mode);
        }
    }
}

/// Configure les paramètres de la fenêtre du jeu.
///
/// # Retourne
/// `Conf` - La configuration définie pour la fenêtre.
fn configuration() -> Conf {
    Conf {
        window_title: "Asteroids Destroyer".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

/// Fonction principale, point d'entrée du programme.
#[macroquad::main(configuration)]
async fn main() {
    menu().await;
}
