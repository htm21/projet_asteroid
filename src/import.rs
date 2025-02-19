use crate::{asteroid::Asteroid, sound::Sound, utils::background_asteroids};
use macroquad::prelude::*;
use rfd::FileDialog;
use std::path::Path;
use std::{fs, path::PathBuf};

/// Menu d'importation de skin.
/// Permet d'importer une image externe au Jeu dans le dossier des skins.
/// Nécessite une image de type PNG uniquement.
///
/// # Arguments
/// - `time` Le temps actuel.
/// - `sound` L'objet Sound qui permettra de jouer les sons associés.
/// - `background_texture` La texture du background.
/// - `asteroid_texture` La texture des astéroids.
/// - `asteroids` Les astéroids.
///
pub async fn import_file(
    time: f64,
    sound: &Sound,
    background_texture: &Texture2D,
    asteroid_texture: &Texture2D,
    asteroids: &mut Vec<Asteroid>,
) {
    let options = ["Asteroid", "Spaceship", "Background"];
    let mut selected_index = 0;

    loop {
        clear_background(BLACK);
        background_asteroids(background_texture, asteroid_texture, asteroids).await;

        let title_dimension = measure_text("Choose an Option", None, 50, 1.0);
        draw_text(
            "Choose an Option",
            (screen_width() - title_dimension.width) / 2.0,
            200.0,
            50.0,
            WHITE,
        );

        for (i, &option) in options.iter().enumerate() {
            let color = if i == selected_index { YELLOW } else { WHITE };
            draw_text(
                option,
                screen_width() / 2.0 - 100.0,
                300.0 + i as f32 * 50.0,
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

        if get_time() - time > 0.2 && is_key_pressed(KeyCode::Enter) {
            sound.play_sound_effect("select_menu");
            match selected_index {
                0 => {
                    if let Err(err) = copy_file_to_folder("assets/pictures/asteroids") {
                        println!("Erreur : {}", err);
                    }
                }
                1 => {
                    if let Err(err) = copy_file_to_folder("assets/pictures/ships") {
                        println!("Erreur : {}", err);
                    }
                }
                2 => {
                    if let Err(err) = copy_file_to_folder("assets/pictures/backgrounds") {
                        println!("Erreur : {}", err);
                    }
                }
                _ => {}
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            sound.play_sound_effect("select_menu");
            return;
        }

        next_frame().await;
    }
}

/// Copie un fichier sélectionné par l'utilisateur dans le chemin de destination.
/// Permet de copier un fichier PNG dans le dossier des skins.
///
/// # Arguments
/// - `destination_folder` Le chemin du dossier de destination.
///
pub fn copy_file_to_folder(destination_folder: &str) -> Result<(), String> {
    let destination_path = Path::new(destination_folder);
    if !destination_path.exists() || !destination_path.is_dir() {
        return Err(format!(
            "Le dossier de destination '{}' n'existe pas ou n'est pas un dossier.",
            destination_folder
        ));
    }

    let selected_file: Option<PathBuf> = FileDialog::new().pick_file();

    match selected_file {
        Some(file_path) => {
            if file_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                != "png"
            {
                return Err(format!(
                    "Le fichier '{}' n'est pas un fichier PNG.",
                    file_path.display()
                ));
            }

            let file_name = file_path.file_name().unwrap_or_default();
            let mut destination_file = destination_path.join(file_name);

            let mut counter = 1;
            while destination_file.exists() {
                let file_stem = file_path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("file");
                let extension = file_path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("png");

                destination_file =
                    destination_path.join(format!("{}_{}.{}", file_stem, counter, extension));
                counter += 1;
            }

            match fs::copy(&file_path, &destination_file) {
                Ok(_) => {
                    println!(
                        "Fichier '{}' copié avec succès vers '{}'.",
                        file_path.display(),
                        destination_file.display()
                    );
                    Ok(())
                }
                Err(e) => Err(format!("Erreur lors de la copie du fichier : {}", e)),
            }
        }
        None => Err("Aucun fichier sélectionné.".to_string()),
    }
}
