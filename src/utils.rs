use crate::asteroid::Asteroid;
use crate::draw_background;
use crate::sound::Sound;
use macroquad::prelude::*;
use std::fs;
use std::time::Instant;

/// Gestion de la touche 'Entrée'.
/// Permet d'éviter les pressions multiples sur la touche 'Entrée'.
///
/// # Arguments
/// - `last_press` Le temps de la dernière pression de touche.
///
/// # Returns
/// - `bool` Vrai si la touche 'Entrée' est pressée après un certain temps.
///
pub fn handle_enter_press(last_press: &mut Instant) -> bool {
    let cooldown = std::time::Duration::from_millis(200);

    if last_press.elapsed() >= cooldown && is_key_pressed(KeyCode::Enter) {
        *last_press = Instant::now();
        return true;
    }
    false
}

pub fn get_top_level_directory_names(dir_path: &str) -> Vec<String> {
    let mut directories = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|name| name.to_str()) {
                    directories.push(dir_name.to_string());
                }
            }
        }
    }
    directories.sort();
    directories
}

/// Permet de choisir une image dans un dossier.
///
/// # Arguments
/// - `folder_path` Le chemin du dossier contenant les images.
/// - `sound` L'objet Sound qui permettra de jouer les sons associés.
///
pub async fn choose_image_from_folder(folder_path: &str, sound: &Sound) -> Option<String> {
    let textures = load_textures_from_folder(folder_path).await;
    let mut last_enter_press = Instant::now();

    if textures.is_empty() {
        eprintln!("Aucune image PNG trouvée dans le dossier.");
        return None;
    }

    let mut selected_index = 0;

    loop {
        clear_background(BLACK);

        let (path, texture) = &textures[selected_index];
        draw_texture_ex(
            texture,
            (screen_width() - texture.width()) / 2.0,
            (screen_height() - texture.height()) / 2.0,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );

        draw_text(
            &format!("Image : {}", path),
            screen_width() / 2.0 - 100.0,
            screen_height() - 30.0,
            20.0,
            WHITE,
        );

        if is_key_pressed(KeyCode::Right) {
            selected_index = (selected_index + 1) % textures.len();
            sound.play_sound_effect("select_menu");
        }
        if is_key_pressed(KeyCode::Left) {
            selected_index = if selected_index == 0 {
                textures.len() - 1
            } else {
                selected_index - 1
            };
            sound.play_sound_effect("select_menu");
        }

        if handle_enter_press(&mut last_enter_press) {
            sound.play_sound_effect("select_menu");
            return Some(textures[selected_index].0.clone());
        }

        next_frame().await;
    }
}

/// Charge une texture à partir d'un fichier.
///
/// # Arguments
/// - `file_path` Le chemin du fichier.
///
/// # Returns
/// - `Texture2D` La texture chargée.
///
async fn load_textures_from_folder(folder_path: &str) -> Vec<(String, Texture2D)> {
    let mut textures = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) == Some("png") {
                if let Ok(texture) = load_texture(path.to_str().unwrap()).await {
                    textures.push((path.to_string_lossy().to_string(), texture));
                } else {
                    eprintln!("Error loading texture: {:?}", path);
                }
            }
        }
    } else {
        eprintln!("Error: Unable to read directory {}", folder_path);
    }

    textures
}

pub fn sigmoid_range_secs(time: f64, t0: f64) -> f64 {
    let l = 1.0;
    let min_range = 0.4;
    let k = 0.05;

    min_range + (l - min_range) / (1.0 + (k * (time - t0)).exp())
}

pub fn sigmoid_speed(time: f64, mode: &str) -> f64 {
    let l = if mode == "classic" { 1.0 } else { 2.5 };
    let k = 0.05;
    let t0 = 30.0;

    1.0 + (l / (1.0 + (-k * (time - t0)).exp()))
}

pub fn has_even_decimal_part(n: f64) -> bool {
    let fractional_part = n.fract();
    let shifted_fraction = (fractional_part * 10.0).round();

    shifted_fraction as u64 % 2 == 0
}

pub async fn background_asteroids(
    background_texture: &Texture2D,
    asteroid_texture: &Texture2D,
    asteroids: &mut Vec<Asteroid>,
) {
    draw_background(background_texture);

    for asteroid in &mut *asteroids {
        asteroid.draw_asteroid(asteroid_texture);
    }
    for asteroid in &mut *asteroids {
        asteroid.move_asteroid(true, 1.0);
    }
}

pub async fn pressed_key(
    background_texture: &Texture2D,
    asteroid_texture: &Texture2D,
    asteroids: &mut Vec<Asteroid>,
) -> KeyCode {
    loop {
        clear_background(BLACK);
        background_asteroids(background_texture, asteroid_texture, asteroids).await;

        let text = "Press a key";
        let dimension = measure_text("Press a key", None, 50, 1.0);
        draw_text(
            text,
            (screen_width() - dimension.width) / 2.0,
            100.0,
            50.0,
            WHITE,
        );
        if let Some(key) = get_last_key_pressed() {
            if key != KeyCode::Enter && key != KeyCode::Backspace && key != KeyCode::Space {
                break key;
            }
        }
        next_frame().await;
    }
}
