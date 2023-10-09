use anyhow::{Context, Result};
use rodio::{source::Buffered, Decoder, Source};
use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{self, File},
    io::BufReader,
    path::Path,
};

pub fn list_available(folder: &str) -> Result<Vec<String>> {
    let items = fs::read_dir(folder).context("Folder do not exists or is unreadable.")?;

    let subdirs: Vec<OsString> = items
        .filter_map(|d| {
            let entry = d.ok()?;
            let path = entry.path();
            if path.is_dir() {
                Some(entry.file_name())
            } else {
                None
            }
        })
        .collect();

    let mut packs: Vec<String> = Vec::new();
    for dir in subdirs.iter() {
        let path = Path::new(folder).join(dir);
        let files = fs::read_dir(path).unwrap();
        let filesnames = files
            .filter_map(|f| {
                let entry = f.ok()?;
                let path = entry.path();
                if path.is_file() {
                    Some(entry.file_name())
                } else {
                    None
                }
            })
            .collect::<Vec<OsString>>();
        let has_config_file = filesnames.contains(&OsString::from("config.json"));
        if has_config_file {
            packs.push(dir.to_str().unwrap().to_owned())
        }
    }

    let subdirs_str: Vec<String> = subdirs
        .iter()
        .map(|d| d.to_str().unwrap().to_owned())
        .collect();

    Ok(subdirs_str)
}

pub fn load_pack(
    folder: &str,
    pack_name: &str,
) -> Result<HashMap<String, Buffered<Decoder<BufReader<File>>>>> {
    let path = Path::new(folder).join(pack_name);
    let config = fs::read_to_string(path.join("config.json"))?;
    let parsed_config: HashMap<String, String> = serde_json::from_str(&config)?;

    let mut final_config: HashMap<String, _> = HashMap::new();
    for (key, value) in parsed_config {
        let file = File::open(path.join(value))?;
        let buf = BufReader::new(file);
        let source = Decoder::new(buf)?;
        let buffered = Decoder::buffered(source);

        final_config.insert(key, buffered);
    }

    Ok(final_config)
}
