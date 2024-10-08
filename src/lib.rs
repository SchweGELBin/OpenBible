use std::collections::HashMap;
use std::error::Error;
use std::fs;

use slint::{self, android};

#[derive(serde::Deserialize)]
struct Verse {
    verse: usize,
    text: String,
}

#[derive(serde::Deserialize)]
struct Chapter {
    name: String,
}

const DATA_DIR: &str = "/storage/emulated/0/Android/data/com.schwegelbin.openbible/files";

// Android Main Function
#[no_mangle]
fn android_main(app: android::AndroidApp) -> Result<(), Box<dyn Error>> {
    android::init(app).unwrap();

    save_index();
    if check_update("schlachter")? {
        download_translation("schlachter");
    }

    ui_text("schlachter", 18, 118);
    Ok(())
}

// Display UI for the translation selection
#[no_mangle]
fn ui_select() {
    slint::slint! {
        export component SelectWindow inherits Window {
            background: black;
        }
    }
    let ui = SelectWindow::new().unwrap();
    ui.run().unwrap();
}

// Display UI fo the text
#[no_mangle]
fn ui_text(abbrev: &str, book: usize, chapter: usize) {
    slint::slint! {
        export component TextWindow inherits Window {
            in property<string> chapter;
            in property<string> text;

            background: black;

            VerticalLayout{
                    width: 96%;

                Rectangle {
                    width: parent.width;
                    height: 50px;

                    Text {
                        text: chapter;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }
                Flickable {
                    width: parent.width;
                    height: 100%;
                    viewport-height: 5500px;

                    Text {
                        text: text;
                        width: parent.width;
                        height: parent.viewport-height;
                        horizontal-alignment: left;
                        vertical-alignment: top;
                    }
                }
            }
        }
    }
    let ui = TextWindow::new().unwrap();

    ui.set_chapter(get_title(&abbrev, book, chapter).unwrap().into());
    ui.set_text(get_chapter(&abbrev, book, chapter).unwrap().into());

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
    if !fs::exists(format!("{}/{}.json", DATA_DIR, &abbrev))? {
        return Ok(true);
    };
    let current = match fs::read_to_string(format!("{}/{}-checksum.json", DATA_DIR, &abbrev)) {
        Ok(data) => data,
        Err(_) => String::new(),
    };
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
    let mut text = String::new();
    let file: String = fs::read_to_string(format!("{}/{}.json", DATA_DIR, &abbrev))?;
    let json: serde_json::Value = serde_json::from_str(&file)?;
    let verses: Vec<Verse> =
        serde_json::from_value(json["books"][book]["chapters"][chapter]["verses"].clone())?;
    println!("{}", json["books"][book]["chapters"][chapter]["name"]);
    for verse in verses {
        text.push_str(format!("{} {}\n", verse.verse, verse.text).as_str());
    }
    Ok(text)
}

// Returns a String of the Title of a chosen chapter
fn get_title(abbrev: &str, book: usize, chapter: usize) -> Result<String, Box<dyn Error>> {
    let file: String = fs::read_to_string(format!("{}/{}.json", DATA_DIR, &abbrev))?;
    let json: serde_json::Value = serde_json::from_str(&file)?;
    let title: Chapter = serde_json::from_value(json["books"][book]["chapters"][chapter].clone())?;
    let title = format!("{}\n{:?}", abbrev, title.name);
    Ok(title)
}
