use crate::asteroid::Asteroid;
use crate::bind::Bind;
use crate::option::option_menu;
use crate::sound::Sound;
use crate::{score, spaceship::Spaceship};
use macroquad::{color, prelude::*};

/// Menu pause du jeu.
/// Permet de mettre en pause le jeu et d'accéder aux réglages (options) du Jeu.
///
/// # Arguments
/// - `background_texture` La texture de l'arrière-plan.
/// - `asteroid_texture` La texture des astéroids.
/// - `sound` Les sons du jeu.
/// - `keybinding` Le Bind actuel.
///
/// # Returns
/// Le temps de pause (pour ne pas le prendre en compte comme 'temps de jeu').
///
pub async fn pause(
    background_texture: &Texture2D,
    asteroid_texture: &Texture2D,
    sound: &mut Sound,
    keybinding: &mut Bind,
) -> f64 {
    if is_key_pressed(KeyCode::Escape) {
        let starting_pause_time = get_time();
        option_menu(
            background_texture,
            asteroid_texture,
            &mut vec![],
            sound,
            keybinding,
            get_time(),
        )
        .await;
        get_time() - starting_pause_time
    } else {
        0.0
    }
}

/// Affichage du bouclier, représenté par des rectangles verts.
/// Lorsque le bouclier diminue, les rectangles verts correspondant disparaissent.
///
/// # Arguments
/// - `shield` Le nombre de rectangles verts à afficher.
///
pub fn draw_shield(shield: u32) {
    for i in 0..shield {
        draw_rectangle(10.0 + 30.0 * i as f32, 70.0, 25.0, 15.0, GREEN);
    }
}

/// Affichage du score.
///
/// # Arguments
/// - `score` Le score de la partie actuelle.
///
pub fn draw_score(score: u32) {
    draw_text(
        &format!("Score : {}", score),
        screen_width() - 180.0,
        80.0,
        30.0,
        WHITE,
    );
}

/// Affichage du nombre d'astéroids.
///
/// # Arguments
/// - `asteroids` Les astéroids de la partie.
///
pub fn draw_asteroid_number(asteroids: &[Asteroid]) {
    draw_text(
        &format!("Asteroid number : {}", asteroids.len()),
        screen_width() - 280.0,
        30.0,
        30.0,
        WHITE,
    );
}

/// Affichage du temps de jeu.
///
pub fn draw_time(time_str: &str) {
    draw_text(time_str, 10.0, 30.0, 30.0, WHITE);
}

/// Affichage du score final.
/// Affiche le score du joueur à la fin de la partie.
///
/// # Arguments
/// - `score` Le score final du joueur.
///
pub fn draw_final_score(score: u32) {
    let text = &format!("Your Score : {}", score);
    let font_size = 40;
    let dimension = measure_text(text, None, font_size, 1.0);
    draw_text(
        text,
        (screen_width() - dimension.width) / 2.0,
        450_f32,
        font_size as f32,
        WHITE,
    );
}

/// Affichage de l'écran de Game Over.
///
/// # Arguments
/// - `spaceship` Le vaisseau du joueur.
/// - `sound` Les sons du jeu (pour jouer le son correspondant).
///
pub async fn game_over(spaceship: &Spaceship, sound: &Sound, win: bool) -> bool {
    let text: &str;
    let color: Color;
    if win {
        sound.play_sound_effect("win");
        text = "You Won !";
        color = Color::from_rgba(0, 213, 109, 1);
    } else {
        text = "Game Over";
        sound.play_sound_effect("gameover");
        color = color::RED;
    }
    loop {
        clear_background(color);
        let font_size = 60;
        let dimension = measure_text(text, None, font_size, 1.0);
        draw_text(
            text,
            (screen_width() - dimension.width) / 2.0,
            200_f32,
            font_size as f32,
            WHITE,
        );

        draw_final_score(score::get_score(spaceship));

        let text2 = "Press 'R' to Restart or 'Esc' to Exit";
        let font_size2 = 40;
        let dimension2 = measure_text(text2, None, font_size2, 1.0);
        draw_text(
            text2,
            (screen_width() - dimension2.width) / 2.0,
            550_f32,
            font_size2 as f32,
            WHITE,
        );

        if is_key_pressed(KeyCode::R) {
            return true;
        }

        if is_key_pressed(KeyCode::Escape) {
            return false;
        }

        next_frame().await;
    }
}
