use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

#[allow(unused_imports)]
use codegen::Scope;
use log::*;
use serde_json::{self, Value};
use walkdir::{DirEntry, WalkDir};

pub mod enums;
pub mod events;
use enums::*;
use events::*;

pub fn find_files(base_path: &str) -> Vec<String> {
    let schema_dir = format!("{base_path}ocsf-schema/");
    debug!("looking for schema files in {schema_dir}");
    let files: Vec<DirEntry> = WalkDir::new(&schema_dir)
        .into_iter()
        .filter_map(|p| p.ok())
        .collect();
    files
        .iter()
        .filter_map(|p| match p.path().is_file() {
            true => Some(p.path().to_string_lossy().into()),
            false => None,
        })
        .collect()
}

#[derive(Debug)]
pub enum ClassPath {
    Enums { class_path: String },
    Event { class_path: String },
    Unknown,
}
pub fn filename_to_classpath(schema_base_path: &str, filename: &str) -> ClassPath {
    let fname = filename.to_owned().replace(schema_base_path, "");
    if fname.starts_with("enums/") {
        let class_path = fname.replace("enums/", "").replace(".json", "");
        return ClassPath::Enums { class_path };
    } else if fname.starts_with("events/") {
        let class_path = fname.replace("event/", "").replace(".json", "");
        return ClassPath::Event { class_path };
    }
    ClassPath::Unknown
}

fn collapsed_title_case(input: &str) -> String {
    let res = input.to_string();
    string_morph::to_title_case(
        &res.replace("enums/", "")
            .replace(".json", "")
            .replace('_', " "),
    )
    .replace(' ', "")
}

pub fn write_source_file(filename: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    // let mut path = PathBuf::from("");
    // path.set_file_name(filename);
    // debug!("trying to write to {:?}", filename);
    // TODO maybe we will need to be appending to this at some point...
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    let written_bytes = writer.write(contents.as_bytes())?;
    debug!("successfully wrote {written_bytes} bytes to {filename}!");
    Ok(())
}

pub fn read_file_to_value(filename: &str) -> Result<Value, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let res: Value = serde_json::from_reader(reader)?;
    Ok(res)
}

pub fn generate_file(
    schema_base_path: &str,
    modules: &mut HashMap<&str, Vec<String>>,
    base_path: &str,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    info!("#########################################");
    info!("Processing {filename:?}");

    let classpath = filename_to_classpath(schema_base_path, filename);
    debug!("ClassPath: {classpath:?}");

    match classpath {
        ClassPath::Enums { class_path } => {
            add_enum(&class_path, base_path, filename)?;
            modules.get_mut("enums").unwrap().push(class_path);
        }
        ClassPath::Event { class_path } => {
            add_event(&class_path, base_path, filename)?;
        }
        ClassPath::Unknown => {
            warn!("Nothing to do yet with {filename:?}!");
        }
    }

    Ok(())
}

fn write_modules(
    ocsf_dir: &str,
    modules: HashMap<&str, Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let enums_dir = format!("{ocsf_dir}src/enums/");
    if !PathBuf::from(&enums_dir).exists() {
        warn!("{enums_dir} is missing, creating it...");
        create_dir_all(&enums_dir)?;
    }
    let events_dir = format!("{ocsf_dir}src/events/");
    if !PathBuf::from(&events_dir).exists() {
        warn!("{events_dir} is missing, creating it...");
        create_dir_all(&events_dir)?;
    }

    let enums_mod_file = File::create(PathBuf::from(format!("{enums_dir}mod.rs")))?;
    let mut enums_mod = BufWriter::new(enums_mod_file);

    let mut enums = modules.get("enums").unwrap().to_vec();
    enums.sort();
    enums_mod.write("\n".as_bytes())?;
    enums.iter().for_each(|e| {
        enums_mod.write_fmt(format_args!("pub mod {e};\n")).unwrap();
    });

    enums.iter().for_each(|e| {
        enums_mod
            .write_fmt(format_args!("pub use {e}::*;\n"))
            .unwrap();
    });

    let events_mod_file = File::create(PathBuf::from(format!("{events_dir}mod.rs")))?;
    let mut events_mod = BufWriter::new(events_mod_file);

    let mut events = modules.get("events").unwrap().to_vec();
    events.sort();
    events_mod.write("\n".as_bytes())?;

    events.iter().for_each(|e| {
        events_mod
            .write_fmt(format_args!("pub mod {e};\n"))
            .unwrap();
    });

    events.iter().for_each(|e| {
        events_mod
            .write_fmt(format_args!("pub use {e}::*;\n"))
            .unwrap();
    });

    Ok(())
}

pub fn generate_scope(base_path: &str) -> Result<(), Box<dyn Error>> {
    let ocsf_dir = format!("{base_path}ocsf/");

    if !PathBuf::from(&ocsf_dir).exists() {
        error!("Dir {ocsf_dir} is missing!");
        panic!();
    }

    // building a list of modules to write out to the parent files later
    let mut modules: HashMap<&str, Vec<String>> = HashMap::new();

    modules.insert("enums", vec![]);
    modules.insert("events", vec![]);

    // find all the schema files
    let mut files = find_files(base_path);
    files.sort();

    for file in files.into_iter() {
        if !file.ends_with(".json") {
            continue;
        }
        if
        // !file.contains("enum") &&
        !file.contains("events") || file.contains("/extensions/")
        // || !file.contains("base_event.json")
        {
            // debug!("Skipping {file}");
            continue;
        }
        match generate_file(
            &format!("{base_path}ocsf-schema/"),
            &mut modules,
            &ocsf_dir,
            &file,
        ) {
            Err(err) => error!("Failed to handle {file}: {err:?}"),
            Ok(_) => info!("[OK] {file}"),
        }
    }

    write_modules(&ocsf_dir, modules)?;

    Ok(())
}
