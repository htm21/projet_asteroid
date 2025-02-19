use macroquad::audio::{self, load_sound, PlaySoundParams, Sound as MacroquadSound};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::fs;

use crate::asteroid::Asteroid;
use crate::utils::background_asteroids;

/// Structure de gestion des sons.
pub struct Sound {
    background_music: Option<MacroquadSound>,
    sound_effects: HashMap<String, MacroquadSound>,
    volume: f32,
    muted: bool,
}

/// Implémentation de la structure Sound.
impl Sound {
    /// Créer une nouvelle structure Sound.
    pub fn new() -> Self {
        Self {
            background_music: None,
            sound_effects: HashMap::new(),
            volume: 1.0,
            muted: false,
        }
    }

    /// Charger la musique de fond.
    ///
    /// # Arguments
    /// - `file_path` Le chemin du fichier de musique de fond.
    ///
    pub async fn load_background_music(&mut self, file_path: &str) {
        match load_sound(file_path).await {
            Ok(sound) => self.background_music = Some(sound),
            Err(e) => eprintln!("Erreur lors du chargement de la musique de fond: {:?}", e),
        }
    }

    /// Joue la musique de fond.
    pub fn play_background_music(&self) {
        if let Some(music) = &self.background_music {
            audio::play_sound(
                music,
                PlaySoundParams {
                    looped: true,
                    volume: self.volume,
                },
            );
        }
    }

    /// Charge un effet sonore et l'ajoute à la liste des effets sonores.
    ///
    /// # Arguments
    /// - `name` Le nom de l'effet sonore qui sera utilisé pour le retrouver.
    /// - `file_path` Le chemin du fichier de l'effet sonore.
    ///
    pub async fn load_sound_effect(&mut self, name: &str, file_path: &str) {
        match load_sound(file_path).await {
            Ok(sound) => {
                self.sound_effects.insert(name.to_string(), sound);
            }
            Err(e) => eprintln!(
                "Erreur lors du chargement de l'effet sonore '{}': {:?}",
                name, e
            ),
        }
    }

    /// Joue un effet sonore (si le son n'est pas "muted"/désactivé).
    ///
    /// # Arguments
    /// - `name` Le nom de l'effet sonore à jouer.
    ///
    pub fn play_sound_effect(&self, name: &str) {
        if self.muted {
            return;
        }
        if let Some(sound) = self.sound_effects.get(name) {
            audio::play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: self.volume,
                },
            );
        } else {
            eprintln!("Aucun effet sonore trouvé avec le nom '{}'", name);
        }
    }

    /// Charge tous les effets sonores d'un dossier et les ajoute à la liste des effets sonores (avec leur nom via `load_sound_effect`).
    ///
    /// # Arguments
    /// - `folder_path` Le chemin du dossier contenant les effets sonores.
    ///
    pub async fn load_all_sounds_from_folder(&mut self, folder_path: &str) {
        if let Ok(entries) = fs::read_dir(folder_path) {
            for entry in entries.flatten() {
                let path = entry.path();

                if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                    if extension == "ogg" || extension == "wav" || extension == "mp3" {
                        if let Some(file_name) = path.file_stem().and_then(|name| name.to_str()) {
                            let file_path = path.to_string_lossy().to_string();
                            self.load_sound_effect(file_name, &file_path).await;
                        }
                    }
                }
            }
        } else {
            eprintln!("Erreur : Impossible de lire le dossier {}", folder_path);
        }
    }

    /// Défini le volume des sons.
    ///
    /// # Arguments
    /// - `volume` Le volume des sons (entre 0.0 et 1.0).
    ///
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        if let Some(music) = &self.background_music {
            audio::stop_sound(music);
            audio::play_sound(
                music,
                PlaySoundParams {
                    looped: true,
                    volume: self.volume,
                },
            );
        }
    }

    /// Active ou désactive le son.
    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        if self.muted {
            audio::stop_sound(self.background_music.as_ref().unwrap());
        } else {
            self.play_background_music();
        }
    }

    /// Vérifie si le son est désactivé.
    pub fn is_muted(&self) -> bool {
        self.muted
    }

    /// Getter du volume actuel.
    pub fn get_volume(&self) -> f32 {
        self.volume
    }
}

/// Menu de configuration du son.
/// Permet à l'utilisateur de régler le volume ou de désactiver le son.
///
/// # Arguments
/// - `sound` La structure Sound à configurer.
/// - `background_texture` La texture de l'arrière-plan.
/// - `asteroid_texture` La texture des astéroids.
/// - `asteroids` Les astéroids.
///
pub async fn setup_sound(
    sound: &mut Sound,
    background_texture: &Texture2D,
    asteroid_texture: &Texture2D,
    asteroids: &mut Vec<Asteroid>,
) {
    loop {
        clear_background(BLACK);
        background_asteroids(background_texture, asteroid_texture, asteroids).await;
        let dimension = measure_text("Sound", None, 50, 1.0);
        draw_text(
            "Sound",
            (screen_width() - dimension.width) / 2.0,
            200.0,
            50.0,
            WHITE,
        );

        let volume_percentage = (sound.get_volume() * 100.0).round();
        let volume_text = format!("Volume: {:.0}%", volume_percentage);

        // Taille de la fenêtre
        let screen_width = screen_width();
        let screen_height = screen_height();

        // Calcul pour centrer les éléments
        let mute_button_width = 150.0;
        let mute_button_height = 40.0;
        let mute_button_x = (screen_width - mute_button_width) / 2.0;
        let mute_button_y = screen_height / 2.0 - 60.0;

        let bar_width = 400.0;
        let bar_height = 20.0;
        let bar_x = (screen_width - bar_width) / 2.0;
        let bar_y = 350.0;

        // Afficher le texte du volume au-dessus de la barre
        let text_width = measure_text(&volume_text, None, 30, 1.0).width;
        draw_text(
            &volume_text,
            (screen_width - text_width) / 2.0,
            bar_y - 20.0,
            30.0,
            WHITE,
        );

        // Dessiner la barre de volume
        draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);

        // Volume actuel (portion remplie)
        let filled_width = (volume_percentage / 100.0) * bar_width;
        draw_rectangle(bar_x, bar_y, filled_width, bar_height, WHITE);

        // Ajuster le volume en cliquant sur la barre
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            if mouse_position.0 >= bar_x
                && mouse_position.0 <= bar_x + bar_width
                && mouse_position.1 >= bar_y
                && mouse_position.1 <= bar_y + bar_height
            {
                let relative_x = mouse_position.0 - bar_x;
                let new_volume = (relative_x / bar_width).clamp(0.0, 1.0);
                sound.set_volume((new_volume * 20.0).round() / 20.0); // Pas de 5
            }
        }

        // Dessiner le bouton "Mute"
        let mute_status = if sound.is_muted() { "Muted" } else { "Unmuted" };
        draw_rectangle(
            mute_button_x,
            mute_button_y,
            mute_button_width,
            mute_button_height,
            GRAY,
        );
        let text_width = measure_text(mute_status, None, 20, 1.0).width;
        draw_text(
            mute_status,
            mute_button_x + (mute_button_width - text_width) / 2.0,
            mute_button_y + (mute_button_height + 15.0) / 2.0,
            20.0,
            WHITE,
        );

        // Vérifier si le bouton "Mute" est cliqué
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            if mouse_position.0 >= mute_button_x
                && mouse_position.0 <= mute_button_x + mute_button_width
                && mouse_position.1 >= mute_button_y
                && mouse_position.1 <= mute_button_y + mute_button_height
            {
                sound.toggle_mute();
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            break;
        }

        next_frame().await;
    }
}
