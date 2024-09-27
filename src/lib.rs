use std::collections::HashMap;
use std::error::Error;
use std::fs;

use slint::{self, android};

#[derive(serde::Deserialize)]
struct Verse {
    verse: usize,
    text: String,
}

const DATA_DIR: &str = "/storage/emulated/0/Android/data/com.schwegelbin.openbible/files";

// Android Main Function
#[no_mangle]
fn android_main(app: android::AndroidApp) {
    android::init(app).unwrap();
    slint::slint! {
        export component MainWindow inherits Window {
            in-out property<string> text;
            Text { text: text; }
        }
    }
    let ui = MainWindow::new().unwrap();

    save_index();
    download_translation("schlachter");
    let update_available = check_update("schlachter");

    ui.set_text(get_chapter("schlachter", 5, 6).unwrap().into());

    ui.run().unwrap();
}

// Saves an index of all translations
fn save_index() -> Result<(), Box<dyn Error>> {
    const TURL: &str = "https://api.getbible.net/v2/translations.json";
    let translations = reqwest::blocking::get(TURL)?.bytes()?;
    fs::write(format!("{}/translations.json", DATA_DIR), &translations)?;
    Ok(())
}

// Downloads a chosen translation and its checksum
fn download_translation(abbrev: &str) -> Result<(), Box<dyn Error>> {
    let turl = format!("https://api.getbible.net/v2/{}.json", &abbrev);
    let translation = reqwest::blocking::get(turl)?.bytes()?;
    let checksum = get_latest_checksum(&abbrev)?;
    fs::write(format!("{}/{}-checksum.json", DATA_DIR, &abbrev), &checksum)?;
    fs::write(format!("{}/{}.json", DATA_DIR, &abbrev), &translation)?;
    Ok(())
}

// Return a bool, wether the online version has a different checksum
fn check_update(abbrev: &str) -> Result<bool, Box<dyn Error>> {
    let latest = get_latest_checksum(&abbrev)?;
    let current = fs::read_to_string(format!("{}/{}-checksum.json", DATA_DIR, &abbrev))?;
    Ok(latest != current)
}

// Helper function to get the online checksum of a chosen translation
fn get_latest_checksum(abbrev: &str) -> Result<String, Box<dyn Error>> {
    const CURL: &str = "https://api.getbible.net/v2/checksum.json";
    let checksum = reqwest::blocking::get(CURL)?.json::<HashMap<String, String>>()?;
    Ok(checksum[abbrev].clone())
}

// Returns a String of every verse of a chosen chapter of a chosen book of a chosen translation
fn get_chapter(abbrev: &str, book: usize, chapter: usize) -> Result<String, Box<dyn Error>> {
    let file: String = fs::read_to_string(format!("{}/{}.json", DATA_DIR, &abbrev))?;
    let json: serde_json::Value = serde_json::from_str(&file)?;
    let mut chapter_string = String::new();
    let verses: Vec<Verse> =
        serde_json::from_value(json["books"][book]["chapters"][chapter]["verses"].clone())?;
    println!("{}", json["books"][book]["chapters"][chapter]["name"]);
    for verse in verses {
        chapter_string.push_str(format!("{}\t{}\n", verse.verse, verse.text).as_str());
    }
    Ok(chapter_string)
}
