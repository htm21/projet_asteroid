use super::play;
use super::skins;
use crate::asteroid::Asteroid;
use crate::bind::Bind;
use crate::option::option_menu;
use crate::sound::Sound;
use crate::utils::background_asteroids;
use macroquad::prelude::*;

/// Menu principal du jeu.
/// Permet le lancement des deux modes de jeu, accès aux options (binding, sons), accès aux choix des skins.
pub async fn menu() {
    // Indice de l'option sélectionnée
    let mut actual_binding = Bind::new();
    let mut selected_index = 0;
    let options = ["Classic Mode", "Modern Mode", "Skins", "Options", "Leave"];
    let mut asteroid_skin = "assets/pictures/asteroids/asteroid.png";
    let mut blackhole_skin = "assets/pictures/blackholes/blackhole.png";
    let mut spaceship_skin = "assets/pictures/ships/spaceship.png";
    let mut background_skin = "assets/pictures/backgrounds/background.png";
    let mut skin_return: (String, String, String, String);
    //Textures
    let mut asteroid_texture = load_texture(asteroid_skin).await.unwrap();
    let mut background_texture = load_texture(background_skin).await.unwrap();
    let mut asteroids: Vec<Asteroid> = (0..10).map(|_| Asteroid::new_random()).collect();
    // Sons
    let mut sound = Sound::new();
    sound.load_all_sounds_from_folder("assets/sounds").await;
    sound
        .load_background_music("assets/sounds/background.wav")
        .await;
    sound.play_background_music();
    sound.play_sound_effect("select_menu");
    loop {
        clear_background(BLACK);
        background_asteroids(&background_texture, &asteroid_texture, &mut asteroids).await;

        // Affichage du titre
        let dimension = measure_text("Asteroid Terrifier 3000", None, 70, 1.0);
        draw_text(
            "Asteroid Terrifier 3000",
            (screen_width() - dimension.width) / 2.0,
            200.0,
            70.0,
            WHITE,
        );

        // Affichage des options
        for (i, &option) in options.iter().enumerate() {
            let color = if i == selected_index { YELLOW } else { WHITE };
            draw_text(
                option,
                screen_width() / 2.0 - 100.0,
                350.0 + i as f32 * 50.0,
                40.0,
                color,
            );
        }

        // Navigation avec les touches fléchées
        if is_key_pressed(KeyCode::Down) {
            selected_index = (selected_index + 1) % options.len();
            sound.play_sound_effect("select_menu");
        }
        if is_key_pressed(KeyCode::Up) {
            selected_index = if selected_index == 0 {
                options.len() - 1
            } else {
                selected_index - 1
            };
            sound.play_sound_effect("select_menu");
        }
        if is_key_pressed(KeyCode::Backspace) {
            sound.play_sound_effect("select_menu");
            return;
        }

        // Actions pour chaque option avec Entrée
        if is_key_pressed(KeyCode::Enter) {
            match selected_index {
                0 => {
                    play(
                        "classic",
                        [
                            asteroid_skin,
                            blackhole_skin,
                            background_skin,
                            spaceship_skin,
                        ],
                        get_time(),
                        &mut sound,
                        &mut actual_binding,
                    )
                    .await;
                }
                1 => {
                    play(
                        "modern",
                        [
                            asteroid_skin,
                            blackhole_skin,
                            background_skin,
                            spaceship_skin,
                        ],
                        get_time(),
                        &mut sound,
                        &mut actual_binding,
                    )
                    .await;
                }
                2 => {
                    skin_return = skins(
                        &background_texture,
                        &asteroid_texture,
                        &mut asteroids,
                        &mut (
                            asteroid_skin.to_string(),
                            background_skin.to_string(),
                            blackhole_skin.to_string(),
                            spaceship_skin.to_string(),
                        ),
                        &sound,
                    )
                    .await;

                    asteroid_skin = &skin_return.0;
                    asteroid_texture = load_texture(asteroid_skin).await.unwrap();
                    background_skin = &skin_return.1;
                    background_texture = load_texture(background_skin).await.unwrap();
                    blackhole_skin = &skin_return.2;
                    spaceship_skin = &skin_return.3;
                }
                3 => {
                    option_menu(
                        &background_texture,
                        &asteroid_texture,
                        &mut asteroids,
                        &mut sound,
                        &mut actual_binding,
                        get_time(),
                    )
                    .await;
                }
                4 => {
                    println!("Leave selected");
                    return; // Quitter le jeu
                }
                _ => {}
            }
        }

        next_frame().await;
    }
}
