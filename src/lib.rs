use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(serde::Deserialize)]
struct Verse {
    verse: usize,
    text: String,
}

// Android Main Function
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    // ... rest of your code ...
    slint::slint! {
        export component MainWindow inherits Window {
            Text { text: "Hello World"; }
        }
    }
    MainWindow::new().unwrap().run().unwrap();
}

// Saves an index of all translations
fn save_index() -> Result<(), Box<dyn Error>> {
    const TURL: &str = "https://api.getbible.net/v2/translations.json";
    let translations = reqwest::blocking::get(TURL)?.bytes()?;
    fs::write("translations.json", &translations)?;
    Ok(())
}

// Downloads a chosen translation and its checksum
fn download_translation(abbrev: &str) -> Result<(), Box<dyn Error>> {
    let turl = format!("https://api.getbible.net/v2/{}.json", &abbrev);
    let translation = reqwest::blocking::get(turl)?.bytes()?;
    let checksum = get_latest_checksum(&abbrev)?;
    fs::write(format!("{}-checksum.json", &abbrev), &checksum)?;
    fs::write(format!("{}.json", &abbrev), &translation)?;
    Ok(())
}

// Return a bool, wether the online version has a different checksum
fn check_update(abbrev: &str) -> Result<bool, Box<dyn Error>> {
    let latest = get_latest_checksum(&abbrev)?;
    let current = fs::read_to_string(format!("{}-checksum.json", &abbrev))?;
    Ok(latest != current)
}

// Helper function to get the online checksum of a chosen translation
fn get_latest_checksum(abbrev: &str) -> Result<String, Box<dyn Error>> {
    const CURL: &str = "https://api.getbible.net/v2/checksum.json";
    let checksum = reqwest::blocking::get(CURL)?.json::<HashMap<String, String>>()?;
    Ok(checksum[abbrev].clone())
}

// Print every verse of a chosen chapter of a chosen book of a chosen translation
fn print_chapter(abbrev: &str, book: usize, chapter: usize) -> Result<(), Box<dyn Error>> {
    let file: String = fs::read_to_string(format!("{}.json", &abbrev))?;
    let json: serde_json::Value = serde_json::from_str(&file)?;
    let verses: Vec<Verse> =
        serde_json::from_value(json["books"][book]["chapters"][chapter]["verses"].clone())?;
    println!("{}", json["books"][book]["chapters"][chapter]["name"]);
    for verse in verses {
        println!("{}\t{}", verse.verse, verse.text);
    }
    Ok(())
}
