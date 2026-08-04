use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{engine::general_purpose::STANDARD, Engine};

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
    pub culture_id: i32,
    pub type_evenement: String,
    pub date: String,
    pub notes: Option<String>,
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
            )", // couleur à faire rentrer dans une range: pour afficher la culture plantée sur le calendrier
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
                notes TEXT
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

pub fn ajouter_evenement(culture_id: Option<i32>, type_event: String, date: String, notes: Option<String>) -> Result<i32> {
    let conn = get_connection()?;
    conn.execute(
        "INSERT INTO evenements (culture_id, type_evenement, date, notes) VALUES (?1, ?2, ?3, ?4)",
        (&culture_id, &type_event, &date, &notes)
    )?;
    Ok(conn.last_insert_rowid() as i32)
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
    let mut stmt = conn.prepare("SELECT id, culture_id, type_evenement, date, notes FROM evenements")?;
    let evenements = stmt
        .query_map((), |row| {
            Ok(Evenement {
                id: row.get(0)?,
                culture_id: row.get(1)?,
                type_evenement: row.get(2)?,
                date: row.get(3)?,
                notes: row.get(4)?,
            })
        })?
        .filter_map(|e| e.ok())
        .collect();
    Ok(evenements)
}

pub fn lister_evenements_par_culture(culture_id: i32) -> Result<Vec<Evenement>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT id, culture_id, type_evenement, date, notes FROM evenements WHERE culture_id = ?1")?;
    let evenements = stmt
        .query_map((&culture_id,), |row| {
            Ok(Evenement {
                id: row.get(0)?,
                culture_id: row.get(1)?,
                type_evenement: row.get(2)?,
                date: row.get(3)?,
                notes: row.get(4)?,
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
        e.id, e.culture_id, e.type_evenement, e.date, e.notes, c.nom, c.variete, c.couleur

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
                culture_nom: row.get(5)?,
                culture_variete: row.get(6)?,
                culture_couleur: row.get(7)?,
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
        e.id, e.culture_id, e.type_evenement, e.date, e.notes, c.nom, c.variete, c.couleur

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
                culture_nom: row.get(5)?,
                culture_variete: row.get(6)?,
                culture_couleur: row.get(7)?,
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
        e.id, e.culture_id, e.type_evenement, e.date, e.notes, c.nom, c.variete, c.couleur

        FROM evenements AS e LEFT JOIN cultures AS c ON e.culture_id = c.id

        WHERE e.date = ?1"

    )?;
    let evenements = stmt
        .query_map((&date,), |row| {
            Ok(EvenementAvecCultureEtImages {
                id: row.get(0)?,
                culture_id: row.get(1)?,
                type_evenement: row.get(2)?,
                date: row.get(3)?,
                notes: row.get(4)?,
                culture_nom: row.get(5)?,
                culture_variete: row.get(6)?,
                culture_couleur: row.get(7)?,
                images: lister_images_par_evenement(row.get(0)?)?,
            })
        })?
        .filter_map(|e| e.ok())
        .collect();
    Ok(evenements)
}   