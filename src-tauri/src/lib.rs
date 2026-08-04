mod db;

use db::Culture;
use db::Evenement;
use db::EvenementAvecCulture;
use db::Image;
use db::EvenementAvecCultureEtImages;

#[tauri::command]
fn ajouter_culture_cmd(nom: String, variete: Option<String>, couleur: String) -> Result<(), String> {
    db::ajouter_culture(nom, variete, couleur).map_err(|e| e.to_string())
}

#[tauri::command]
fn ajouter_evenement_cmd(culture_id: Option<i32>, type_event: String, date: String, notes: Option<String>) -> Result<i32, String> {
    db::ajouter_evenement(culture_id, type_event, date, notes).map_err(|e| e.to_string())
}

#[tauri::command]
fn ajouter_image_cmd(evenement_id: i32, nom_fichier: String, donnees: String) -> Result<(), String> {
    db::ajouter_image(evenement_id, nom_fichier, donnees).map_err(|e| e.to_string())
}

#[tauri::command]
fn lister_cultures_cmd() -> Result<Vec<Culture>, String> {
    db::lister_cultures().map_err(|e| e.to_string())
}

#[tauri::command]
fn lister_evenements_cmd() -> Result<Vec<Evenement>, String> {
    db::lister_evenements().map_err(|e| e.to_string())
}

#[tauri::command]
fn lister_evenements_par_culture_cmd(culture_id: i32) -> Result<Vec<Evenement>, String> {
    db::lister_evenements_par_culture(culture_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn lister_evenements_par_date_cmd(date: String) -> Result<Vec<EvenementAvecCulture>, String> {
    db::lister_evenements_par_date(date).map_err(|e| e.to_string())
}

#[tauri::command]
fn lister_evenements_avec_culture_cmd() -> Result<Vec<EvenementAvecCulture>, String> {
    db::lister_evenements_avec_culture().map_err(|e| e.to_string())
}

#[tauri::command]
fn lister_images_par_evenement_cmd(evenement_id: i32) -> Result<Vec<Image>, String> {
    db::lister_images_par_evenement(evenement_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn lister_evenements_par_date_avec_culture_et_images_cmd(date: String) -> Result<Vec<EvenementAvecCultureEtImages>, String> {
    db::lister_evenements_par_date_avec_culture_et_images(date).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    //Ajouter dans le invoke handler les fonctions que l'on souhaite exposer à l'interface utilisateur
    .invoke_handler(tauri::generate_handler![
        ajouter_culture_cmd, 
        lister_cultures_cmd, 
        ajouter_evenement_cmd, 
        ajouter_image_cmd, 
        lister_evenements_cmd, 
        lister_evenements_par_culture_cmd, 
        lister_evenements_par_date_cmd, 
        lister_evenements_avec_culture_cmd, 
        lister_images_par_evenement_cmd, 
        lister_evenements_par_date_avec_culture_et_images_cmd
        ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Autorise l'affichage des images copiées dans images/ (à côté de
      // potager.db) via convertFileSrc, sans dépendre d'un chemin figé qui
      // diffèrerait entre `tauri dev` et le binaire buildé.
      use tauri::Manager;
      let dossier_images = std::env::current_dir()?.join("images");
      std::fs::create_dir_all(&dossier_images)?;
      app.asset_protocol_scope().allow_directory(&dossier_images, true)?;

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}