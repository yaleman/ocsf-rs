use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::sync::LazyLock;

use chrono::Utc;
use codegen::Scope;
use itertools::Itertools;
use log::*;
use serde_json::{self, Value};
use walkdir::{DirEntry, WalkDir};

pub mod categories;
pub mod dictionary;
pub mod enums;
pub mod errors;
pub mod events;
pub mod module;
pub mod objects;
pub mod other;
pub mod profiles;
use categories::*;
use dictionary::*;
use enums::*;
pub use errors::*;
use events::*;
use objects::*;
use other::*;
use profiles::*;

// #[allow(dead_code)]
// static URL_FINDER: Lazy<Regex> =
// Lazy::new(|| Regex::new(r#"(?P<url>\w+://[^<\s]+)"#).expect("Failed to generate URL regex"));

#[allow(dead_code)]
type ClassesHashMap = HashMap<&'static str, HashMap<String, ClassType>>;

#[allow(clippy::large_enum_variant)]
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

    if let Ok(existing_file) = &mut File::open(filename) {
        debug!("Found an existing file in place!");
        let mut file_contents = String::new();
        existing_file.read_to_string(&mut file_contents)?;

        if without_timestamp_line(file_contents) == without_timestamp_line(contents.to_string()) {
            debug!("Don't need to write a new file to {filename}, contents already match other than timestamp!");
            return Ok(());
        }
    }

    let file = File::create(filename)?;

    let mut writer = BufWriter::new(file);
    let written_bytes = writer.write(contents.as_bytes())?;
    debug!("Successfully wrote {written_bytes} bytes to {filename}");
    Ok(())
}

pub fn get_new_scope_with_comment(first_line: Option<String>, paths: &DirPaths) -> Scope {
    let mut new_scope = Scope::new();
    if let Some(comment) = first_line {
        new_scope.raw(comment);
    }

    new_scope.add_generation_timestamp_comment(get_schema_version(paths).unwrap());
    new_scope
}

/// uses serde_json to try and parse a given file
pub fn read_file_to_value(filename: &str) -> Result<Value, Box<dyn Error>> {
    debug!("read_file_to_value {filename}");
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
        if e.is_empty() {
            panic!("Empty module name?");
        }
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
        if e.is_empty() {
            panic!("Empty module name?");
        }
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
    /// destination path + `src/`
    fn source_path(&self) -> PathBuf {
        PathBuf::from(&format!("{}src/", &self.destination_path))
    }
}

pub fn get_timestamp_matcher() -> regex::Regex {
    regex::Regex::new(r#"^(// This file was automatically generated by ocsf-codegen at .*)"#)
        .unwrap()
}

/// output the code with the timestamp line stripped
pub fn without_timestamp_line(input: String) -> String {
    get_timestamp_matcher().replace_all(&input, "").into_owned()
}

pub trait CustomScopeThings {
    fn writeln(&mut self, line: impl std::fmt::Display);
    fn add_generation_timestamp_comment(&mut self, schema_version: String);
}

impl CustomScopeThings for Scope {
    /// Writes a raw string to the scope but doesn't forget to add the newline this time.
    fn writeln(&mut self, line: impl std::fmt::Display) {
        self.raw(line.to_string());
    }

    fn add_generation_timestamp_comment(&mut self, schema_version: String) {
        let timestamp = Utc::now();

        let commit = last_git_commit::LastGitCommit::new().build().unwrap();

        self.writeln(format!(
            "// This file was automatically generated by ocsf-codegen at {} branch: \"{}\" link: <https://github.com/yaleman/ocsf-rs/commit/{}> OCSF Schema version: {}",
            timestamp.to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
            commit.branch(),
            commit.id().long(),
            schema_version,
        ));
    }
}

/// Used when cleaning up docstrings
static TAG_REPLACEMENTS: LazyLock<Vec<(&str, &str)>> = LazyLock::new(|| {
    vec![
        ("<ul>", ""),
        ("</ul>", ""),
        ("<li>", "\n/// * "),
        ("<b>", "**"),
        ("</b>", "**"),
        ("<p>", "\n/// "),
        ("</p>", "\n///\n ///"),
        ("<code>", "`"),
        ("</code>", "`"),
    ]
});

/// Strips a bunch of stuff out
pub fn fix_docstring(input: String, leading_docstring: Option<&'static str>) -> String {
    let comment = leading_docstring.unwrap_or("///");

    let mut input = input;

    for (tag, replacement) in TAG_REPLACEMENTS.iter() {
        input = input.replace(tag, replacement);
    }
    input = input.replace("</p>", &format!("\n{}", comment));
    while input.ends_with("\n") {
        let _ = input.strip_suffix("\n");
    }
    while input.ends_with("\r") {
        let _ = input.strip_suffix("\r");
    }
    input
}

fn generate_expected_paths(paths: &DirPaths, modules: &[String]) -> Vec<String> {
    let mut ok_paths: Vec<String> = vec![];

    modules.iter().for_each(|m| {
        ok_paths.push(format!("{}src/{}.rs", paths.destination_path, m));
        ok_paths.push(format!("{}src/{}/", paths.destination_path, m));
    });

    ok_paths.push(format!("{}src/", paths.destination_path));
    ok_paths.push(format!("{}src/lib.rs", paths.destination_path));
    ok_paths
}

/// checks that all the expected files are there, and if not then it's
fn check_crate_files(paths: &DirPaths, ok_paths: Vec<String>) -> Result<(), &'static str> {
    debug!("OK Paths:{:#?}", ok_paths.iter().sorted());

    let mut found_bad_files = false;

    // let's double-check that any files we expect actually exist...
    for filename in walkdir::WalkDir::new(format!("{}src/", paths.destination_path)) {
        let filename = filename.unwrap();

        let mut file_type = "file";
        if filename.path().is_dir() {
            file_type = "directory";
        }

        let filename_str = filename.path().to_str().unwrap().to_string();

        if !ok_paths.contains(&filename_str) {
            error!("module has unexpected {file_type}: {filename_str}");
            found_bad_files = true;
        } else {
            trace!("Found expected crate source file file: {filename_str}");
        }
    }

    match found_bad_files {
        false => Ok(()),
        true => Err("found something bad, you should check that!"),
    }
}

pub fn generate_template_files(base_path: &str) -> Result<(), Box<dyn Error>> {
    for file in glob::glob("./ocsf-codegen/templates/*").unwrap().flatten() {
        let file_name = file
            .strip_prefix("ocsf-codegen/templates/")
            .unwrap()
            .to_path_buf();
        println!("{:?}", file_name);

        let destination_filepath = PathBuf::from(base_path).join(file_name);
        println!("Writing {:?} to {:?}", file, destination_filepath);

        let new_file = File::create(destination_filepath)?;
        let mut new_file = BufWriter::new(new_file);

        let file_contents = std::fs::read_to_string(&file)?;
        new_file.write_all(file_contents.as_bytes())?;
    }
    Ok(())
}

/// main function of the library that generates the `ocsf` crate.
pub fn generate_source_code(base_path: &str) -> Result<(), Box<dyn Error>> {
    let paths = DirPaths::new(base_path);

    if !PathBuf::from(&paths.destination_path).exists() {
        error!("Dir {} is missing!", paths.destination_path);
        panic!();
    }

    generate_template_files(&paths.destination_path)?;

    let src_dir = PathBuf::from(&format!("{}/src/", &paths.destination_path));
    if !src_dir.exists() {
        std::fs::create_dir(src_dir)?;
    }

    let mut root_module = module::Module::new("lib".to_string(), true);

    root_module.scope = Scope::new();
    root_module
        .scope
        .raw("//! OCSF crate, does Open Cyber Security Framework things.");
    root_module.scope.raw("//! ");
    root_module
        .scope
        .raw("//! <h1><span color=\"red\"> THIS IS VERY VERY VERY EARLY ALPHA</span></h1>");
    root_module.scope.raw("//! ");
    root_module
        .scope
        .raw("//! The base schema is available at <https://ocsf.io>.");
    root_module
        .scope
        .add_generation_timestamp_comment(get_schema_version(&paths).unwrap());
    root_module
        .scope
        .raw("#![allow(rustdoc::invalid_html_tags)]");
    let modules = [
        "categories",
        "dictionary",
        // "enums",
        "events",
        "objects",
        // "other",
        "profiles",
    ];
    let modules: Vec<String> = modules.iter().map(|f| f.to_string()).collect();

    for module_name in modules.iter() {
        root_module.add_child(module_name.to_string());
    }

    add_version_element(&paths, &mut root_module.scope)?;

    generate_enums(&paths, &mut root_module)?;

    generate_dictionary_entries(&paths, &mut root_module)?;
    generate_profiles(&paths, &mut root_module)?;
    generate_categories(&paths, &mut root_module)?;
    generate_objects(&paths, &mut root_module)?;
    generate_events(&paths, &mut root_module)?;

    let mut expected_paths: Vec<String> = generate_expected_paths(&paths, &modules);
    root_module.write_module(
        &mut expected_paths,
        &paths.source_path(),
        get_schema_version(&paths)?,
    )?;
    check_crate_files(&paths, expected_paths)?;

    Ok(())
}
