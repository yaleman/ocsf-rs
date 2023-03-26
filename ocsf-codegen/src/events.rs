use std::error::Error;
use std::path::Path;
// use std::path::Path;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Map;

use crate::module::Module;
use crate::*;
use glob::glob;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EventDef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub attributes: HashMap<String, EventAttribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
}

/// this finds an event schema file based on its name and returns the contents - or panics if not
#[allow(dead_code)]
fn find_event_schema_file(base_path: &str, name: &str) -> String {
    let search_string = format!("{base_path}events/**/*.json");
    debug!("Looking for object called {name} in {search_string}");
    for filename in glob(&search_string).unwrap().flatten() {
        let filename_str = filename.to_str().unwrap();

        let file_contents = match read_file_to_value(filename_str) {
            Ok(val) => val,
            Err(err) => {
                error!("Failed to parse {filename_str}: {err:?}");
                panic!();
            }
        };
        let object_name = file_contents.get("name").unwrap().as_str().unwrap();
        if object_name == name {
            info!("Success! {:?}", filename_str);
            return filename_str.to_string();
        } else {
            debug!(
                "Name from {} didn't match: {} != {}",
                filename_str, object_name, name
            );
            // panic!();
        }
    }
    // This is a panic-level event because we've got schema files relying on others that don't exist
    panic!("Didn't find {} in {}", name, search_string);
    // None
}

#[allow(dead_code)]
fn handle_attribute_includes(
    _base_path: &str,
    _module_source_path: &str,
    _filename: &str,
    includes: Vec<String>,
) {
    includes
        .iter()
        .for_each(|i| warn!("Need to write include handler for {i}"))
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Group {
    Classification,
    Context,
    Occurrence,
    Primary,
}

impl<'de> Deserialize<'de> for Group {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let stringval = String::deserialize(deserializer)?.to_lowercase();
        let result: Group = stringval.as_str().into();
        Ok(result)
    }
}

impl From<&str> for Group {
    fn from(value: &str) -> Self {
        match value {
            "classification" => Self::Classification,
            "context" => Self::Context,
            "occurrence" => Self::Occurrence,
            "primary" => Self::Primary,
            _ => panic!("Invalid enum value {value} - select from classification, context, occurrence, primary")
        }
    }
}
impl From<Group> for &'static str {
    fn from(input: Group) -> &'static str {
        match input {
            Group::Classification => "classification",
            Group::Context => "context",
            Group::Occurrence => "occurrence",
            Group::Primary => "primary",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Requirement {
    Optional,
    Recommended,
    Required,
}

impl<'de> Deserialize<'de> for Requirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let stringval = String::deserialize(deserializer)?.to_lowercase();
        let result: Requirement = stringval.as_str().into();
        Ok(result)
    }
}

impl From<&str> for Requirement {
    fn from(value: &str) -> Self {
        match value {
            "optional" => Self::Optional,
            "recommended" => Self::Recommended,
            "required" => Self::Required,
            _ => panic!("Invalid enum value {value} - select from optional,recommended, =required"),
        }
    }
}

impl From<Requirement> for &'static str {
    fn from(input: Requirement) -> &'static str {
        match input {
            Requirement::Optional => "optional",
            Requirement::Recommended => "recommended",
            Requirement::Required => "required",
        }
    }
}

#[test]
fn test_from_str_requirement() {
    use crate::events::*;
    assert_eq!(Requirement::from("required"), Requirement::Required);
}

#[test]
#[should_panic]
fn test_from_str_invalid_requirement() {
    use crate::events::*;
    let _ = Requirement::from("requiasdfasdfred");
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventAttribute {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requirement: Option<Requirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<Group>,
    #[serde(alias = "$include", skip_serializing_if = "Option::is_none")]
    include: Option<String>,
}

impl EventAttribute {
    pub fn new(name: String) -> Self {
        EventAttribute {
            name: Some(name),
            ..Self::default()
        }
    }
}

impl Default for EventAttribute {
    fn default() -> Self {
        Self {
            name: Some("".to_string()),
            caption: Default::default(),
            description: None,
            group: Default::default(),
            profile: Default::default(),
            requirement: Default::default(),
            include: Default::default(),
        }
    }
}

impl EventAttribute {
    pub fn name(self, name: String) -> Self {
        Self {
            name: Some(name),
            ..self
        }
    }
    pub fn profile(self, profile: Option<&str>) -> Self {
        Self {
            profile: profile.map(|val| val.to_string()),
            ..self
        }
    }
    pub fn description(self, description: String) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }
    pub fn caption(self, value: String) -> Self {
        Self {
            caption: Some(value),
            ..self
        }
    }
    pub fn requirement(self, value: Requirement) -> Self {
        Self {
            requirement: Some(value),
            ..self
        }
    }
    pub fn group(self, value: Group) -> Self {
        Self {
            group: Some(value),
            ..self
        }
    }
}

/// returns an [EventAttribute] and  list of attribute names, so I can track down what I need to support :D
#[allow(dead_code)]
fn handle_attribute(
    _base_path: &str,
    _module_source_path: &str,
    filename: &str,
    attribute_name: &str,
    attribute: Map<String, Value>,
) -> EventAttribute {
    let attrkeys: Vec<String> = attribute.keys().map(|k| k.to_string()).collect();
    info!(
        "Handling attribute {} (keys: {:#?})",
        attribute_name,
        attrkeys.join(",")
    );

    let mut result = EventAttribute::new(attribute_name.to_string());

    attribute.iter().for_each(|(key, value)| {
        info!("attr: {} val: {:?}", key, value);
        result = match key.to_owned().as_str() {
            "$include" => {
                // TODO: handle includes inside attributes!
                warn!(
                    "Attribute {} in {} needs include: {}",
                    attribute_name,
                    filename,
                    value.as_str().unwrap()
                );
                result.clone()
            }
            "caption" => result.clone().caption(value.as_str().unwrap().into()),
            "description" => result.clone().description(value.as_str().unwrap().into()),
            "enum" => {
                warn!(
                    "Attribute {} in {} needs enum: {:?}",
                    attribute_name,
                    filename,
                    value.to_string()
                );
                result.clone() // TODO: handle arbitrary enums inside of events!
            }
            "group" => result.clone().group(value.as_str().unwrap().into()),
            "profile" => {
                debug!("profile value: {:?}", value.as_str());
                result.clone().profile(value.as_str())
            }
            "requirement" => result.clone().requirement(value.as_str().unwrap().into()),
            _ => {
                warn!("Unhandled attr key: {key:?}");
                result.clone()
            }
        };
    });
    debug!("{result:#?}");
    result
}

pub fn generate_events(paths: &DirPaths, root_module: &mut Module) -> Result<(), Box<dyn Error>> {
    let event_schema_path = format!("{}events/", paths.schema_path);
    let filenames = find_files(&event_schema_path);

    let mut module_map: HashMap<String, Scope> = HashMap::new();
    module_map.insert(
        "mod.rs".to_string(),
        get_new_scope_with_comment(Some("//! Events Module for the OCSF crate".to_string())),
    );

    for file in filenames {
        let stripped_file = file.replace(&event_schema_path, "");
        let filepath_split: Vec<String> = stripped_file.split('/').map(|f| f.to_string()).collect();
        let mut module_filename = "mod.rs".to_string();
        let filename = filepath_split.last().unwrap().to_owned();
        if filepath_split.len() > 1 {
            let res: Vec<String> = filepath_split[0..(filepath_split.len() - 1)].to_vec();
            module_filename = format!("{}.rs", res.join("/"));

            let module_name = module_filename.split('.').next().unwrap();
            match module_name.contains('/') {
                true => {
                    // we're in a nested module

                    let splitmodule: Vec<&str> = module_name.split('/').collect();
                    if splitmodule.len() > 2 {
                        panic!("can't handle events modules with multiple levels of nesting...?");
                    };

                    let parent_module_name =
                        format!("{}.rs", module_name.split('/').next().unwrap());
                    let parent_module = module_map.get_mut(&parent_module_name).unwrap();
                    let module_def = format!("pub mod {};", module_name.split('/').last().unwrap());
                    if !parent_module.to_string().contains(&module_def) {
                        // we need to add the module def
                        parent_module.raw(module_def);
                    }
                }
                false => {
                    // we can add to the base module
                    let base_module = module_map.get_mut("mod.rs").unwrap();
                    let module_def = format!("pub mod {};", module_name);
                    if !base_module.to_string().contains(&module_def) {
                        // we need to add the module def
                        base_module.raw(module_def);
                    }
                }
            };
        }

        if !module_map.contains_key(&module_filename) {
            module_map.insert(module_filename.clone(), get_new_scope_with_comment(None));
        }

        let mut event = read_file_to_value(&file)?;
        let mut attribute_file_includes: Vec<Value> = vec![];

        // let's pull the attribute $includes out so as not to upset the apple cart.
        if let Some(attributes) = event.as_object_mut().unwrap().get_mut("attributes") {
            if let Some(include_vals) = attributes.as_object_mut().unwrap().remove("$include") {
                if include_vals.is_array() {
                    let include_vals_array = include_vals.as_array().unwrap();
                    attribute_file_includes.extend(include_vals_array.to_owned());
                } else if include_vals.is_string() {
                    attribute_file_includes.push(include_vals.to_owned());
                }
            }
        }
        #[allow(unused_variables)]
        let attribute_includes: Vec<String> = attribute_file_includes
            .into_iter()
            .map(|i| i.as_str().unwrap().to_string())
            .collect();

        let mut event: EventDef = serde_json::from_value(event)?;
        // TODO: attribute value includes.

        let output_scope = module_map.get_mut(&module_filename).unwrap();

        // TODO: deal with attribute *internal* includes - there's one in registry_key.json
        event.attributes.iter_mut().for_each(|(_key, attrib)| {
            if let Some(include_filename) = attrib.include.clone() {
                info!("need to include {} in {}", include_filename, &filename);
                // because we haven't dealt with them yet!
                if !include_filename.starts_with("enums/") {
                    panic!(
                        "Attribute in {filename} trying to include {include_filename} and it's not an enum!"
                    );
                }
                let include_file =
                    read_file_to_value(&format!("{}{}", paths.schema_path, include_filename)).unwrap();
                // let include_file = include_file.as_object().unwrap();
                debug!("included file: {include_file:#?}");
                let enum_name = collapsed_title_case(include_filename.replace(".json", "").split('/').last().unwrap());

                // TODO: see if we can figure out if the enum already exists over in enums, because it probably already does!

                match
                enum_from_value(
                    paths,
                    root_module,
                     include_file,
                     enum_name.clone(),
                     ) {
                        Ok(val) => root_module.enums.push(val),
                        Err(err) => debug!("Tried to add {}: {}", enum_name, err),
                     };
            };
        });

        if filename == "registry_key.json" {
            debug!("{} -> {}", filename, serde_json::to_string_pretty(&event)?);
            return Ok(());
        }

        output_scope.raw(&format!("// kilroy was here {filename}"));

        // debug!("Module filename: {module_filename}");
    }

    // writing out the module files to disk
    module_map.iter().for_each(|(filename, scope)| {
        let full_path = format!("{}src/events/{}", paths.destination_path, filename);
        // create the dirs if they don't exist
        let parent_dir = Path::new(&full_path).parent().unwrap();
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }

        write_source_file(&full_path, &scope.to_string()).unwrap();
    });

    Ok(())
}
