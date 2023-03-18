use std::error::Error;

use serde_json::{json, Map};

use crate::*;
use glob::glob;

#[derive(Clone, Debug, Default)]
pub struct EventDef {
    pub uid: Option<u32>,
    pub class_name: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub attributes: Vec<EventAttribute>,
    pub associations: Map<String, Value>,
    pub profiles: Vec<String>,
}

pub fn generate_event_modules(paths: &DirPaths) -> Result<(), Box<dyn Error>> {

    // building a list of modules to write out to the parent files later
    let mut modules: HashMap<&str, Vec<String>> = HashMap::new();
    let mut classes: ClassesHashMap = HashMap::new();

    modules.insert("enums", vec![]);
    modules.insert("events", vec![]);

    classes.insert("enums", HashMap::new());
    classes.insert("events", HashMap::new());

    // find all the schema files
    let mut files = find_files(&paths.schema_path);
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
        match process_file(
            &paths.schema_path,
            &mut modules,
            &mut classes,
            &paths.destination_path,
            &file,
        ) {
            Err(err) => error!("Failed to handle {file}: {err:?}"),
            Ok(_) => info!("[OK] {file}"),
        }
    }

    write_modules(&paths.destination_path, modules)
}

/// this finds an event schema file based on its name and returns the contents - or panics if not
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

// trait EventDataTrait {
//     fn handle_event_extends(&mut self, schema_path: &str, modules: &mut HashMap<&str, Vec<String>>)  -> Self;
// }

// impl EventDataTrait for Map<String, Value> {
//     fn handle_event_extends(&mut self, schema_path: &str, modules: &mut HashMap<&str, Vec<String>>)  -> Self {
//         let extend_val = self.get("extends").unwrap().as_str().unwrap();
//         warn!("Extends issued for {}: {}", self.get("name").unwrap().to_string(), extend_val);
//         if modules["events"].contains(&extend_val.to_string()) {
//             info!("Already loaded this one, woo!");
//         } else {
//             info!("Haven't Already loaded {}", extend_val);
//             find_event_schema_file(schema_path, extend_val);
//         }
//         // TODO: we need to basically load the event and return that... but we probably haven't parsed *that* event yet!
//         self.to_owned()
//     }
// }

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Group {
    Classification,
    Context,
    Occurrence,
    Primary,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Requirement {
    Optional,
    Recommended,
    Required,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventAttribute {
    name: String,
    profile: Option<String>,
    description: String,
    caption: Option<String>,
    requirement: Option<Requirement>,
    group: Option<Group>,
}

impl EventAttribute {
    pub fn new(name: String) -> Self {
        EventAttribute {
            name,
            ..Self::default()
        }
    }
}

impl Default for EventAttribute {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            caption: Default::default(),
            description: "No description provided".to_string(),
            group: Default::default(),
            profile: Default::default(),
            requirement: Default::default(),
        }
    }
}

impl EventAttribute {
    pub fn name(self, name: String) -> Self {
        Self { name, ..self }
    }
    pub fn profile(self, profile: Option<&str>) -> Self {
        Self {
            profile: profile.map(|val| val.to_string()),
            ..self
        }
    }
    pub fn description(self, description: String) -> Self {
        Self {
            description,
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

pub fn add_event(
    // modules: &mut HashMap<&str, Vec<String>>,
    classes: &mut ClassesHashMap,
    // TODO: rename this to be sensible
    base_path: &str, // this is the base path of the event file, it's terrible
    module_source_path: &str,
    schema_base_path: &str,
    filename: &str,
) -> Result<EventDef, Box<dyn Error>> {
    debug!("Module source path: {}", module_source_path);

    let file_object = read_file_to_value(filename).unwrap();
    if !file_object.is_object() {
        error!("Not sure what this is!");
        error!("{:?}", file_object);
        panic!();
    }
    let base_path = base_path.replace("events/", "");
    let event_name = collapsed_title_case(&base_path);

    info!(
        "Adding event called {} from {} to {}",
        event_name, filename, base_path
    );

    if event_name.contains('/') {
        warn!("Uh, event names can't have / in them!");
    }

    let file_object = file_object.as_object().unwrap().to_owned();
    debug!("{:#?}", file_object);

    let mut result: EventDef = match file_object.get("extends") {
        Some(extend_val) => {
            let extend_schema_name = extend_val.as_str().unwrap();

            let mut start_point: EventDef = EventDef::default();
            if let Some(val) = classes.get("events").unwrap().get(extend_schema_name) {
                // panic!("found it!");
                if let ClassType::Event { value } = val {
                    start_point = value.clone();
                }
            } else {
                let extend_schema_filename =
                    find_event_schema_file(schema_base_path, extend_schema_name);
                start_point = add_event(
                    classes,
                    &base_path,
                    module_source_path,
                    schema_base_path,
                    &extend_schema_filename,
                )?;
                debug!("Extending from base of:\n:{start_point:#?}");
                classes.get_mut("events").unwrap().insert(
                    extend_schema_name.to_string(),
                    ClassType::Event {
                        value: start_point.clone(),
                    },
                );
            }

            start_point
        }
        None => EventDef::default(),
    };

    result.description = file_object
        .get("description")
        .unwrap_or(&json!("No description was included in the schema."))
        .as_str()
        .unwrap()
        .to_string();
    result.name = file_object
        .get("name")
        .expect("No 'name' field was in the schema definition!")
        .as_str()
        .unwrap()
        .to_string();

    result.category = match file_object.get("category") {
        Some(val) => val.as_str().unwrap().to_string(),
        None => match file_object.get("extends") {
            None => panic!("No category or extends in this file!"),
            Some(val) => val.as_str().unwrap().to_string(),
        },
    };

    // TODO: work out if the profiles are additive when you extend from them
    result.profiles = file_object
        .get("profiles")
        .unwrap_or(&json!(Vec::<Value>::new()))
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p.to_string())
        .collect();

    let attributes = file_object
        .get("attributes")
        .unwrap()
        .as_object()
        .unwrap()
        .to_owned();
    if let Some(includes) = attributes.get("$include") {
        let mut includes_list: Vec<String> = vec![];

        if includes.is_array() {
            includes_list.extend(includes.as_array().unwrap().iter().map(|i| i.to_string()));
        } else if includes.is_string() {
            includes_list.push(includes.to_string());
        }

        handle_attribute_includes(&base_path, module_source_path, filename, includes_list)
    }

    result.associations = match attributes.get("associations") {
        Some(val) => val.as_object().unwrap().to_owned(),
        None => Map::new(),
    };

    // let mut attribute_keys: Vec<String> = vec![];
    let mut event_attributes = vec![];

    attributes.iter().for_each(|(attribute_name, attribute)| {
        if attribute_name == "$include" {
            return;
        }
        let attribute = attribute.as_object().unwrap().to_owned();
        let event_attribute = handle_attribute(
            &base_path,
            module_source_path,
            filename,
            attribute_name,
            attribute,
        );
        // for key in attrkeys {
        //     if !attribute_keys.contains(&key) {
        //         attribute_keys.push(key);
        //     }
        // }
        event_attributes.push(event_attribute);
    });

    result.class_name = collapsed_title_case(file_object.get("name").unwrap().as_str().unwrap());
    result.attributes.extend(event_attributes);

    // info!("Seen attribute keys:");
    // attribute_keys.iter().for_each(|k| info!("attrkey {k}"));

    // check that the schema's all done
    let handled_fields: Vec<String> = [
        "description",
        "profiles",
        "name",
        "category",
        "caption", // TODO: do we care about the caption?
        "attributes",
        "associations",
        "uid",
        "extends",
    ]
    .into_iter()
    .map(|f| f.to_string())
    .collect();

    file_object.iter().for_each(|(k, v)| {
        if !handled_fields.contains(k) {
            warn!("Unhandled field in event schema: {k}");
            warn!("Unhandled field in event schema: {k} => {v}");
        }
    });

    result.uid = file_object
        .get("uid")
        .map(|val| val.as_u64().unwrap() as u32);

    debug!("{result:#?}");

    Ok(result)
}
