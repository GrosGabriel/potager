use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{Duration, NaiveDate};

#[derive(Serialize, Deserialize)]
pub struct Culture {
    pub id: i32,
    pub nom: String,
    pub variete: Option<String>,
    pub couleur: String,
}

#[derive(Serialize, Deserialize)]
pub struct Evenement {
    pub id: i32,
    pub culture_id: Option<i32>,
    pub type_evenement: String,
    pub date: String,
    pub notes: Option<String>,
    pub temperature_int: Option<f32>,
    pub temperature_ext: Option<f32>,
    pub temps_arrosage: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub evenement_id: i32,
    pub chemin_fichier: String,
}

#[derive(Serialize, Deserialize)]
pub struct EvenementAvecCulture {
    pub id: i32,
    pub culture_id: Option<i32>,
    pub type_evenement: String,
    pub date: String,
    pub notes: Option<String>,
    pub temperature_int: Option<f32>,
    pub temperature_ext: Option<f32>,
    pub temps_arrosage: Option<i32>,
    pub culture_nom: Option<String>,
    pub culture_variete: Option<String>,
    pub culture_couleur: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EvenementAvecCultureEtImages {
    pub id: i32,
    pub culture_id: Option<i32>,
    pub type_evenement: String,
    pub date: String,
    pub notes: Option<String>,
    pub temperature_int: Option<f32>,
    pub temperature_ext: Option<f32>,
    pub temps_arrosage: Option<i32>,
    pub culture_nom: Option<String>,
    pub culture_variete: Option<String>,
    pub culture_couleur: Option<String>,
    pub images: Vec<Image>,
}

pub fn get_connection() -> Result<Connection> {
    let mut conn = Connection::open("potager.db")?;
    let tx = conn.transaction()?;
    tx.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY
        )",
        (),
    )?;

    let version_actuelle: i32 = tx.query_row(
        "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
        (), |row| row.get(0)).unwrap_or(0);
    
    if version_actuelle < 1 {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS cultures (
                id INTEGER PRIMARY KEY,
                nom TEXT NOT NULL,
                variete TEXT,
                couleur TEXT
            )", 
            (),
        )?;
        tx.execute(
            "INSERT INTO schema_version (version) VALUES (1)", ()
        )?;
    }

    if version_actuelle < 2 {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS evenements (
                id INTEGER PRIMARY KEY,
                culture_id INTEGER REFERENCES cultures(id),
                type_evenement TEXT NOT NULL,
                date TEXT NOT NULL,
                notes TEXT,
                temperature_int REAL,
                temperature_ext REAL,
                temps_arrosage INTEGER
            )",
            (),
        )?;
        tx.execute(
            "INSERT INTO schema_version (version) VALUES (2)", ()
        )?;
    }
    
    if version_actuelle < 3 {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS images (
                id INTEGER PRIMARY KEY,
                evenement_id INTEGER REFERENCES evenements(id),
                chemin_fichier TEXT NOT NULL
            )",
            (),
        )?;
        tx.execute(
            "INSERT INTO schema_version (version) VALUES (3)", ()
        )?;
    }
    tx.commit()?;
    Ok(conn)
}

pub fn ajouter_culture(nom: String, variete: Option<String>, couleur: String) -> Result<()> {
    let conn = get_connection()?;
    conn.execute(
        "INSERT INTO cultures (nom, variete, couleur) VALUES (?1, ?2, ?3)",
        (&nom, &variete, &couleur)
    )?;
    Ok(())
}

pub fn ajouter_evenement(culture_id: Option<i32>, type_event: String, date: String, notes: Option<String>, temperature_int: Option<f32>, temperature_ext: Option<f32>, temps_arrosage: Option<i32>) -> Result<i32> {
    let conn = get_connection()?;
    conn.execute(
        "INSERT INTO evenements (culture_id, type_evenement, date, notes, temperature_int, temperature_ext, temps_arrosage) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&culture_id, &type_event, &date, &notes, &temperature_int, &temperature_ext, &temps_arrosage)
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

/// Crée le premier événement (avec ses notes/températures) puis, dans la
/// même transaction, un événement nu par jour suivant jusqu'à `nombre_jours`
/// jours au total (premier jour inclus). Si une insertion échoue en cours de
/// route, rien n'est validé : la transaction est abandonnée (rollback
/// automatique tant que `commit()` n'a pas été appelé).
pub fn ajouter_evenement_avec_repetition(
    culture_id: Option<i32>,
    type_event: String,
    date: String,
    notes: Option<String>,
    temperature_int: Option<f32>,
    temperature_ext: Option<f32>,
    temps_arrosage: Option<i32>,
    nombre_jours: i32,
) -> Result<i32> {
    let mut conn = get_connection()?;
    let tx = conn.transaction()?;

    tx.execute(
        "INSERT INTO evenements (culture_id, type_evenement, date, notes, temperature_int, temperature_ext, temps_arrosage) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&culture_id, &type_event, &date, &notes, &temperature_int, &temperature_ext, &temps_arrosage)
    )?;
    let premier_id = tx.last_insert_rowid() as i32;

    let date_depart = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(erreur_io)?;
    for i in 1..nombre_jours {
        let date_suivante = (date_depart + Duration::days(i as i64)).format("%Y-%m-%d").to_string();
        tx.execute(
            "INSERT INTO evenements (culture_id, type_evenement, date, temps_arrosage) VALUES (?1, ?2, ?3, ?4)",
            (&culture_id, &type_event, &date_suivante, &temps_arrosage)
        )?;
    }

    tx.commit()?;
    Ok(premier_id)
}

/// Dossier où sont copiées les images, à côté de potager.db (résolu depuis le
/// répertoire courant, comme la base : voir get_connection).
fn dossier_images() -> PathBuf {
    PathBuf::from("images")
}

fn erreur_io(e: impl std::error::Error + Send + Sync + 'static) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(e))
}

/// Copie les données (envoyées en base64 par le frontend, en clair ou en
/// data URL "data:image/png;base64,...") dans le dossier images/, puis
/// enregistre le chemin relatif obtenu en base.
pub fn ajouter_image(evenement_id: i32, nom_fichier: String, donnees_base64: String) -> Result<()> {
    let donnees_base64 = donnees_base64.rsplit(',').next().unwrap_or(&donnees_base64);
    let octets = STANDARD.decode(donnees_base64).map_err(erreur_io)?;

    let dossier = dossier_images();
    fs::create_dir_all(&dossier).map_err(erreur_io)?;

    let nom_original = Path::new(&nom_fichier)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("image");
    let horodatage = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(erreur_io)?
        .as_nanos();
    let nom_unique = format!("{evenement_id}_{horodatage}_{nom_original}");
    fs::write(dossier.join(&nom_unique), &octets).map_err(erreur_io)?;

    let chemin_relatif = format!("images/{nom_unique}");
    let conn = get_connection()?;
    conn.execute(
        "INSERT INTO images (evenement_id, chemin_fichier) VALUES (?1, ?2)",
        (&evenement_id, &chemin_relatif)
    )?;
    Ok(())
}


#[derive(Serialize, Deserialize)]
pub struct NombreEvenementsParType {
    pub journal: i32,
    pub arrosage: i32,
    pub recolte: i32,
    pub plantation: i32,
    pub semis: i32,
    pub retrait: i32,
    pub temperature: i32,
}

pub fn nombre_evenements_par_type() -> Result<NombreEvenementsParType> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT type_evenement, COUNT(*) FROM evenements GROUP BY type_evenement")?;
    let mut compteurs = NombreEvenementsParType {
        journal: 0,
        arrosage: 0,
        recolte: 0,
        plantation: 0,
        semis: 0,
        retrait: 0,
        temperature: 0,
    };
    let lignes = stmt.query_map((), |row| {
        let type_evenement: String = row.get(0)?;
        let nombre: i32 = row.get(1)?;
        Ok((type_evenement, nombre))
    })?;
    for ligne in lignes {
        let (type_evenement, nombre) = ligne?;
        match type_evenement.as_str() {
            "Journal" => compteurs.journal = nombre,
            "Arrosage" => compteurs.arrosage = nombre,
            "Récolte" => compteurs.recolte = nombre,
            "Plantation" => compteurs.plantation = nombre,
            "Semis" => compteurs.semis = nombre,
            "Retrait" => compteurs.retrait = nombre,
            "Température" => compteurs.temperature = nombre,
            _ => {}
        }
    }
    Ok(compteurs)
}

pub fn lister_cultures() -> Result<Vec<Culture>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT id, nom, variete, couleur FROM cultures")?;
    let cultures = stmt
        .query_map((), |row| {
            Ok(Culture {
                id: row.get(0)?,
                nom: row.get(1)?,
                variete: row.get(2)?,
                couleur: row.get(3)?,
            })
        })?
        .filter_map(|c| c.ok())
        .collect();
    Ok(cultures)
}

pub fn lister_evenements() -> Result<Vec<Evenement>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT id, culture_id, type_evenement, date, notes, temperature_int, temperature_ext, temps_arrosage FROM evenements")?;
    let evenements = stmt
        .query_map((), |row| {
            Ok(Evenement {
                id: row.get(0)?,
                culture_id: row.get(1)?,
                type_evenement: row.get(2)?,
                date: row.get(3)?,
                notes: row.get(4)?,
                temperature_int: row.get(5)?,
                temperature_ext: row.get(6)?,
                temps_arrosage: row.get(7)?,
            })
        })?
        .filter_map(|e| e.ok())
        .collect();
    Ok(evenements)
}

pub fn lister_evenements_par_culture(culture_id: i32) -> Result<Vec<Evenement>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT id, culture_id, type_evenement, date, notes, temperature_int, temperature_ext, temps_arrosage FROM evenements WHERE culture_id = ?1")?;
    let evenements = stmt
        .query_map((&culture_id,), |row| {
            Ok(Evenement {
                id: row.get(0)?,
                culture_id: row.get(1)?,
                type_evenement: row.get(2)?,
                date: row.get(3)?,
                notes: row.get(4)?,
                temperature_int: row.get(5)?,
                temperature_ext: row.get(6)?,
                temps_arrosage: row.get(7)?,
            })
        })?
        .filter_map(|e| e.ok())
        .collect();
    Ok(evenements)
}

pub fn lister_evenements_par_date(date: String) -> Result<Vec<EvenementAvecCulture>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT
        e.id, e.culture_id, e.type_evenement, e.date, e.notes, e.temperature_int, e.temperature_ext, e.temps_arrosage, c.nom, c.variete, c.couleur

        FROM evenements AS e LEFT JOIN cultures AS c ON e.culture_id = c.id

        WHERE e.date = ?1"

    )?;
    let evenements = stmt
        .query_map((&date,), |row| {
            Ok(EvenementAvecCulture {
                id: row.get(0)?,
                culture_id: row.get(1)?,
                type_evenement: row.get(2)?,
                date: row.get(3)?,
                notes: row.get(4)?,
                temperature_int: row.get(5)?,
                temperature_ext: row.get(6)?,
                temps_arrosage: row.get(7)?,
                culture_nom: row.get(8)?,
                culture_variete: row.get(9)?,
                culture_couleur: row.get(10)?,
            })
        })?
        .filter_map(|e| e.ok())
        .collect();
    Ok(evenements)
}

pub fn lister_evenements_avec_culture() -> Result<Vec<EvenementAvecCulture>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT
        e.id, e.culture_id, e.type_evenement, e.date, e.notes, e.temperature_int, e.temperature_ext, e.temps_arrosage, c.nom, c.variete, c.couleur

        FROM evenements AS e LEFT JOIN cultures AS c ON e.culture_id = c.id"

    )?;
    let evenements = stmt
        .query_map((), |row| {
            Ok(EvenementAvecCulture {
                id: row.get(0)?,
                culture_id: row.get(1)?,
                type_evenement: row.get(2)?,
                date: row.get(3)?,
                notes: row.get(4)?,
                temperature_int: row.get(5)?,
                temperature_ext: row.get(6)?,
                temps_arrosage: row.get(7)?,
                culture_nom: row.get(8)?,
                culture_variete: row.get(9)?,
                culture_couleur: row.get(10)?,
            })
        })?
        .filter_map(|e| e.ok())
        .collect();
    Ok(evenements)
}

pub fn lister_images_par_evenement(evenement_id: i32) -> Result<Vec<Image>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT id, evenement_id, chemin_fichier FROM images WHERE evenement_id = ?1")?;
    let images = stmt
        .query_map((&evenement_id,), |row| {
            Ok(Image {
                id: row.get(0)?,
                evenement_id: row.get(1)?,
                chemin_fichier: row.get(2)?,
            })
        })?
        .filter_map(|i| i.ok())
        .collect();
    Ok(images)
}


pub fn lister_evenements_par_date_avec_culture_et_images(date: String) -> Result<Vec<EvenementAvecCultureEtImages>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT
        e.id, e.culture_id, e.type_evenement, e.date, e.notes, e.temperature_int, e.temperature_ext, e.temps_arrosage, c.nom, c.variete, c.couleur

        FROM evenements AS e LEFT JOIN cultures AS c ON e.culture_id = c.id

        WHERE e.date = ?1

        ORDER BY e.id ASC"

    )?;
    let evenements = stmt
        .query_map((&date,), |row| {
            Ok(EvenementAvecCultureEtImages {
                id: row.get(0)?,
                culture_id: row.get(1)?,
                type_evenement: row.get(2)?,
                date: row.get(3)?,
                notes: row.get(4)?,
                temperature_int: row.get(5)?,
                temperature_ext: row.get(6)?,
                temps_arrosage: row.get(7)?,
                culture_nom: row.get(8)?,
                culture_variete: row.get(9)?,
                culture_couleur: row.get(10)?,
                images: lister_images_par_evenement(row.get(0)?)?,
            })
        })?
        .filter_map(|e| e.ok())
        .collect();
    Ok(evenements)
}   

pub fn supprimer_evenement(evenement_id: i32) -> Result<()> {
    // Supprimer d'abord les fichiers images copiés sur disque, avant les
    // lignes en base qui gardent trace de leur chemin.
    for image in lister_images_par_evenement(evenement_id)? {
        let _ = fs::remove_file(&image.chemin_fichier);
    }

    let conn = get_connection()?;
    conn.execute("DELETE FROM images WHERE evenement_id = ?1", (&evenement_id,))?;
    conn.execute("DELETE FROM evenements WHERE id = ?1", (&evenement_id,))?;
    Ok(())
}

pub fn supprimer_culture(culture_id: i32) -> Result<()> {
    let conn = get_connection()?;
    // Supprimer d'abord les événements liés à cette culture, et leurs images.
    for evenement in lister_evenements_par_culture(culture_id)? {
        supprimer_evenement(evenement.id)?;
    }
    conn.execute("DELETE FROM cultures WHERE id = ?1", (&culture_id,))?;
    Ok(())
}


pub fn nombre_images() -> Result<i32> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM images")?;
    let nombre: i32 = stmt.query_row((), |row| row.get(0))?;
    Ok(nombre)
}
