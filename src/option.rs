use crate::asteroid::Asteroid;
use crate::bind::{binding, Bind};
use crate::sound::{setup_sound, Sound};
use crate::utils::background_asteroids;
use macroquad::prelude::*;

/// Menu des options.
/// Permet d'accèder aux réglages du binding des touches et du niveau du son.
///
/// # Arguments
/// - `background_texture` La texture de l'arrière-plan.
/// - `asteroid_texture` La texture des astéroids.
/// - `asteroids` Les astéroids.
/// - `sound` Les sons du jeu.
/// - `actual_binding` Le Bind actuel.
/// - `last_press` Le temps de la dernière pression de touche (pour éviter les pressions multiples).
///
pub async fn option_menu(
    background_texture: &Texture2D,
    asteroid_texture: &Texture2D,
    asteroids: &mut Vec<Asteroid>,
    sound: &mut Sound,
    actual_binding: &mut Bind,
    last_press: f64,
) {
    let mut selected_index = 0;
    let options = ["Binding", "Sound", "Back"];

    loop {
        clear_background(BLACK);
        background_asteroids(background_texture, asteroid_texture, asteroids).await;

        let dimension = measure_text("Options", None, 50, 1.0);
        draw_text(
            "Options",
            (screen_width() - dimension.width) / 2.0,
            200.0,
            50.0,
            WHITE,
        );

        for (i, &option) in options.iter().enumerate() {
            let color = if i == selected_index { YELLOW } else { WHITE };
            let dimension = measure_text(option, None, 40, 1.0);
            if option == "Back" {
                draw_text(
                    option,
                    (screen_width() - dimension.width) / 2.0,
                    screen_height() - 350.0,
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
        if get_time() - last_press > 0.2 {
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
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Backspace) {
                sound.play_sound_effect("select_menu");
                return;
            }

            if is_key_pressed(KeyCode::Enter) {
                match selected_index {
                    0 => {
                        binding(
                            background_texture,
                            asteroid_texture,
                            asteroids,
                            sound,
                            actual_binding,
                        )
                        .await;
                    }
                    1 => {
                        setup_sound(sound, background_texture, asteroid_texture, asteroids).await;
                    }
                    2 => {
                        sound.play_sound_effect("select_menu");
                        return;
                    }
                    _ => {}
                }
            }
        }
        next_frame().await;
    }
}
