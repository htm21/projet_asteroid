use crate::{asteroid::Asteroid, import::import_file, sound::Sound};

use super::utils::*;
use macroquad::prelude::*;
use std::time::Instant;

/// Menu de sélection des skins.
/// Permet de choisir les skins des astéroids, du background, des trous noirs et du vaisseau.
///
/// # Arguments
/// - `background_texture` La texture du background.
/// - `asteroid_texture` La texture des astéroids.
/// - `asteroids` Les astéroids de la partie.
/// - `skins` Les skins.
/// - `sound` L'objet Sound qui permettra de jouer les sons associés.
///
/// # Returns
/// - Un tuples contenant les chemins des skins choisis (asteroids, background, blackholes, spaceship).
///
pub async fn skins(
    background_texture: &Texture2D,
    asteroid_texture: &Texture2D,
    asteroids: &mut Vec<Asteroid>,
    skins: &mut (String, String, String, String),
    sound: &Sound,
) -> (String, String, String, String) {
    let mut background_texture = background_texture.clone();
    let mut asteroid_texture = asteroid_texture.clone();

    let mut selected_index = 0;
    let mut options = get_top_level_directory_names("assets/pictures/");
    options.push("Import".to_string());
    options.push("Back".to_string());
    let mut last_enter_press = Instant::now();

    sound.play_sound_effect("select_menu");

    loop {
        clear_background(BLACK);
        background_asteroids(&background_texture, &asteroid_texture, asteroids).await;

        let dimension = measure_text("Skins", None, 50, 1.0);

        draw_text(
            "Skins",
            (screen_width() - dimension.width) / 2.0,
            200.0,
            50.0,
            WHITE,
        );

        for (i, option) in options.iter().enumerate() {
            let color = if i == selected_index { YELLOW } else { WHITE };
            let dimension = measure_text(option, None, 40, 1.0);
            if option == "Back" {
                draw_text(
                    option,
                    (screen_width() - dimension.width) / 2.0,
                    screen_height() - 200.0,
                    40.0,
                    color,
                );
            } else {
                draw_text(
                    option,
                    (screen_width() - dimension.width) / 2.0,
                    350.0 + i as f32 * 50.0,
                    40.0,
                    color,
                );
            }
        }

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

        if is_key_down(KeyCode::Backspace) {
            sound.play_sound_effect("select_menu");
            return skins.clone();
        }

        if handle_enter_press(&mut last_enter_press) {
            match selected_index {
                0 => {
                    skins.0 = choose_image_from_folder("assets/pictures/asteroids", sound)
                        .await
                        .unwrap();
                    asteroid_texture = load_texture(&skins.0).await.unwrap();
                }
                1 => {
                    skins.1 = choose_image_from_folder("assets/pictures/backgrounds", sound)
                        .await
                        .unwrap();
                    background_texture = load_texture(&skins.1).await.unwrap();
                }
                2 => {
                    skins.2 = choose_image_from_folder("assets/pictures/blackholes", sound)
                        .await
                        .unwrap();
                }
                3 => {
                    skins.3 = choose_image_from_folder("assets/pictures/ships", sound)
                        .await
                        .unwrap();
                }
                4 => {
                    import_file(
                        get_time(),
                        sound,
                        &background_texture,
                        &asteroid_texture,
                        asteroids,
                    )
                    .await;
                }
                5 => {
                    return skins.clone();
                }
                _ => {}
            }
        }

        next_frame().await;
    }
}
