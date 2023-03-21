#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use chrono::Utc;
use codegen::Scope;
use log::*;
use regex::Regex;
use serde_json::{self, Value};
use walkdir::{DirEntry, WalkDir};

pub mod categories;
pub mod dictionary;
pub mod enums;
pub mod events;
pub mod objects;
pub mod other;
pub mod profiles;
use categories::*;
use dictionary::*;
use enums::*;
use events::*;
use objects::*;
use other::*;
use profiles::*;

lazy_static! {
    static ref URL_FINDER: Regex = Regex::new(r#"(?P<url>\w+://[^<\s]+)"#).unwrap();
}

#[allow(dead_code)]
type ClassesHashMap = HashMap<&'static str, HashMap<String, ClassType>>;

#[derive(Debug)]
pub enum ClassType {
    Event { value: EventDef },
    Enum { value: EnumDef },
}

#[derive(Debug)]
pub enum ClassPath {
    Enums { class_path: String },
    Event { class_path: String },
    Unknown,
}
pub fn find_files(schema_path: &str) -> Vec<String> {
    debug!("looking for schema files in {schema_path}");
    let files: Vec<DirEntry> = WalkDir::new(schema_path)
        .into_iter()
        .filter_map(|p| p.ok())
        .collect();
    info!("Found {} files to process.", files.len());
    files
        .iter()
        .filter_map(|p| match p.path().is_file() {
            true => Some(p.path().to_string_lossy().into()),
            false => None,
        })
        .collect()
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

fn collapsed_title_case(input: impl std::fmt::Display) -> String {
    let res = input.to_string();
    string_morph::to_title_case(
        &res.replace("enums/", "")
            .replace(".json", "")
            .replace('_', " "),
    )
    .replace(' ', "")
}

/// write a file to a place
pub fn write_source_file(filename: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    debug!("Writing source file to {filename}");
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    let written_bytes = writer.write(contents.as_bytes())?;
    debug!("Successfully wrote {written_bytes} bytes to {filename}");
    Ok(())
}

pub fn get_new_scope_with_comment(first_line: Option<String>) -> Scope {
    let mut new_scope = Scope::new();
    if let Some(comment) = first_line {
        new_scope.raw(&comment);
    }
    new_scope.add_generation_timestamp_comment();
    new_scope
}

/// uses serde_json to try and parse a given file
pub fn read_file_to_value(filename: &str) -> Result<Value, Box<dyn Error>> {
    trace!("read_file_to_value {filename}");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let res: Value = serde_json::from_reader(reader)?;
    Ok(res)
}

#[allow(dead_code)]
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
    let _ = enums_mod.write("\n".as_bytes())?;
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
    let _ = events_mod.write("\n".as_bytes())?;

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

pub struct DirPaths {
    pub destination_path: String,
    pub schema_path: String,
}

impl DirPaths {
    fn new(base_path: &str) -> Self {
        Self {
            destination_path: format!("{base_path}ocsf/"),
            schema_path: format!("{base_path}ocsf-schema/"),
        }
    }
}

/// generates a timestamp comment to use while writing files
pub fn generate_timestamp_comment() -> String {
    let timestamp = Utc::now();
    format!(
        "// This file was automatically generated by ocsf-codegen at {}",
        timestamp.to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
    )
}

pub trait CustomScopeThings {
    fn writeln(&mut self, line: impl std::fmt::Display);
    fn add_generation_timestamp_comment(&mut self);
}

impl CustomScopeThings for Scope {
    /// Writes a raw string to the scope but doesn't forget to add the newline this time.
    fn writeln(&mut self, line: impl std::fmt::Display) {
        self.raw(&format!("{line}\n"));
    }
    fn add_generation_timestamp_comment(&mut self) {
        self.writeln(&generate_timestamp_comment());
    }
}

/// Strips a bunch of stuff out
pub fn fix_docstring(input: String, leading_docstring: Option<&'static str>) -> String {
    let comment = leading_docstring.unwrap_or("///");

    input
        .replace("<b>", "**")
        .replace("</b>", "**")
        .replace("<p>", "")
        .replace("</p>", &format!("\n{}", comment))
        .replace("<code>", "`")
        .replace("</code>", "`")
}

/// main function of the library that generates the `ocsf` crate.
pub fn generate_source_code(base_path: &str) -> Result<(), Box<dyn Error>> {
    let paths = DirPaths::new(base_path);

    if !PathBuf::from(&paths.destination_path).exists() {
        error!("Dir {} is missing!", paths.destination_path);
        panic!();
    }

    let mut output_scope = Scope::new();
    output_scope.raw("//! OCSF crate, does Open Cyber Security Framework things.");
    output_scope.raw("//! ");
    output_scope.raw("//! <h1><span color=\"red\"> THIS IS VERY VERY VERY EARLY ALPHA</span></h1>");
    output_scope.raw("//! ");
    output_scope.raw("//! The base schema is available at <https://ocsf.io>.");
    output_scope.add_generation_timestamp_comment();

    for module_name in [
        "categories",
        "dictionary",
        "enums",
        "events",
        "objects",
        // "other",
        "profiles",
    ] {
        output_scope.raw(&format!("pub mod {};", module_name));
    }

    add_version_element(&paths, &mut output_scope)?;
    // generate_event_modules(&paths)?;
    generate_dictionary_entries(&paths)?;
    generate_profiles(&paths)?;
    // generate_other(&paths)?;
    generate_categories(&paths)?;
    generate_enums(&paths)?;
    generate_events(&paths)?;

    generate_objects(&paths)?;
    write_source_file(
        &format!("{}src/lib.rs", paths.destination_path),
        &output_scope.to_string(),
    )?;

    Ok(())
}
