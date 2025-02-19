use crate::asteroid::Asteroid;
use crate::sound::Sound;
use crate::utils::{background_asteroids, handle_enter_press, pressed_key};
use macroquad::prelude::*;
use std::time::Instant;

/// Structure des Binds de jeu.
pub struct Bind {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
}

/// Implémentation du Bind.
impl Bind {
    /// Créer un Bind initial.
    /// Par défaut, les touches sont les flèches directionnelles.
    ///
    /// # Returns
    /// - `Bind` Nouveau Bind.
    ///
    pub fn new() -> Bind {
        Bind {
            up: KeyCode::Up,
            down: KeyCode::Down,
            left: KeyCode::Left,
            right: KeyCode::Right,
        }
    }

    /// Setters des touches de déplacement.
    pub fn set_up(&mut self, up: KeyCode) {
        self.up = up;
    }

    pub fn set_down(&mut self, down: KeyCode) {
        self.down = down;
    }

    pub fn set_left(&mut self, left: KeyCode) {
        self.left = left;
    }

    pub fn set_right(&mut self, right: KeyCode) {
        self.right = right;
    }

    /// Getters des touches de déplacement.
    pub fn get_up(&self) -> KeyCode {
        self.up
    }

    pub fn get_down(&self) -> KeyCode {
        self.down
    }
    pub fn get_left(&self) -> KeyCode {
        self.left
    }

    pub fn get_right(&self) -> KeyCode {
        self.right
    }
}

/// Menu de Binding des touches de déplacement.
/// Permet de changer les touches de déplacement du vaisseau spatial avec une interaction utilisateur.
/// Possibilité de réinitialiser les touches par défaut.
///
/// # Arguments
/// - `background_texture` La texture de l'arrière-plan.
/// - `asteroid_texture` La texture des astéroids.
/// - `asteroids` Les astéroids.
/// - `sound` Les sons du jeu.
/// - `actual_binding` Le Bind actuel.
///
pub async fn binding(
    background_texture: &Texture2D,
    asteroid_texture: &Texture2D,
    asteroids: &mut Vec<Asteroid>,
    sound: &Sound,
    actual_binding: &mut Bind,
) {
    let mut selected_index = 0;
    let options = ["UP", "DOWN", "LEFT", "RIGHT", "RESET", "Back"];
    let mut last_press = Instant::now();
    loop {
        clear_background(BLACK);
        background_asteroids(background_texture, asteroid_texture, asteroids).await;

        let dimension = measure_text("Binding", None, 50, 1.0);
        draw_text(
            "Binding",
            (screen_width() - dimension.width) / 2.0,
            200.0,
            50.0,
            WHITE,
        );

        for (i, &option) in options.iter().enumerate() {
            let color = if i == selected_index { YELLOW } else { WHITE };
            if option == "Back" {
                if option == "Back" {
                    let text = "Back";
                    let dimension = measure_text(text, None, 40, 1.0);
                    draw_text(
                        option,
                        (screen_width() - dimension.width) / 2.0,
                        screen_height() - 200.0,
                        40.0,
                        color,
                    );
                    continue;
                }
            } else if option == "RESET" {
                let text = "Reset";
                let dimension = measure_text(text, None, 40, 1.0);
                draw_text(
                    text,
                    (screen_width() - dimension.width) / 2.0,
                    350.0 + i as f32 * 50.0,
                    40.0,
                    color,
                );
                continue;
            }
            let text = format!(
                "Bind for {}: {:?}",
                option,
                match i {
                    0 => actual_binding.get_up(),
                    1 => actual_binding.get_down(),
                    2 => actual_binding.get_left(),
                    3 => actual_binding.get_right(),
                    _ => KeyCode::Unknown,
                }
            );
            let dimension = measure_text(&text, None, 40, 1.0);
            draw_text(
                &text,
                (screen_width() - dimension.width) / 2.0,
                350.0 + i as f32 * 50.0,
                40.0,
                color,
            );
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
        if is_key_pressed(KeyCode::Backspace) {
            sound.play_sound_effect("select_menu");
            return;
        }

        if handle_enter_press(&mut last_press) {
            match selected_index {
                0 => {
                    actual_binding
                        .set_up(pressed_key(background_texture, asteroid_texture, asteroids).await);
                }
                1 => {
                    actual_binding.set_down(
                        pressed_key(background_texture, asteroid_texture, asteroids).await,
                    );
                }
                2 => {
                    actual_binding.set_left(
                        pressed_key(background_texture, asteroid_texture, asteroids).await,
                    );
                }
                3 => {
                    actual_binding.set_right(
                        pressed_key(background_texture, asteroid_texture, asteroids).await,
                    );
                }
                4 => {
                    actual_binding.set_up(KeyCode::Up);
                    actual_binding.set_down(KeyCode::Down);
                    actual_binding.set_left(KeyCode::Left);
                    actual_binding.set_right(KeyCode::Right);
                }
                5 => {
                    return;
                }

                _ => {}
            }
        }
        next_frame().await;
    }
}
