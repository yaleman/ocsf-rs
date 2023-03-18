use std::error::Error;

use serde_json::{json, Map};

use crate::*;

pub struct EventDef {
    pub uid: Option<u32>,
    pub name: String,
    pub category: String,
    pub attribute_keys: Vec<String>,
}

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
    profile: Option<String>,
    description: String,
    caption: Option<String>,
    requirement: Option<Requirement>,
    group: Option<Group>,
}

impl Default for EventAttribute {
    fn default() -> Self {
        Self {
            profile: Default::default(),
            description: "No description provided".to_string(),
            caption: Default::default(),
            requirement: Default::default(),
            group: Default::default(),
        }
    }
}

impl EventAttribute {
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

fn handle_attribute(
    _base_path: &str,
    _module_source_path: &str,
    filename: &str,
    attribute_name: &str,
    attribute: Map<String, Value>,
) -> Vec<String> {
    let attrkeys: Vec<String> = attribute.keys().map(|k| k.to_string()).collect();
    info!(
        "Handling attribute {} (keys: {:#?})",
        attribute_name,
        attrkeys.join(",")
    );

    let mut result = EventAttribute::default();

    attribute.iter().for_each(|(key, value)| {
        info!("attr: {} val: {:?}", key, value);
        result = match key.to_owned().as_str() {
            "$include" => {
                // TODO: handle includes inside attributes!
                debug!(
                    "Attribute {} in {} needs include: {}",
                    attribute_name,
                    filename,
                    value.as_str().unwrap()
                );
                result.clone()
            }
            "description" => result.clone().description(value.as_str().unwrap().into()),
            "enum" => {
                debug!(
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
    attrkeys
}

pub fn add_event(
    base_path: &str,
    module_source_path: &str,
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

    let file_object = read_file_to_value(filename).unwrap();
    if !file_object.is_object() {
        error!("Not sure what this is!");
        error!("{:?}", file_object);
        panic!();
    }

    let file_object = file_object.as_object().unwrap().to_owned();
    debug!("{:#?}", file_object);

    let description = file_object
        .get("description")
        .unwrap_or(&json!("No description was included in the schema."))
        .as_str()
        .unwrap()
        .to_string();
    let name = file_object
        .get("name")
        .expect("No 'name' field was in the schema definition!")
        .as_str()
        .unwrap()
        .to_string();
    let category: String = match file_object.get("category") {
        Some(val) => val.as_str().unwrap().to_string(),
        None => match file_object.get("extends") {
            None => panic!("No category or extends in this file!"),
            Some(val) => val.as_str().unwrap().to_string(),
        },
    };

    // .expect("No 'catgory' field was in the schema definition!").as_str().unwrap().to_string();

    let profiles: Vec<String> = file_object
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

    let attribute_keys: Vec<Vec<String>> = attributes
        .iter()
        .map(|(attribute_name, attribute)| {
            if attribute_name == "$include" {
                return vec![];
            }
            let attribute = attribute.as_object().unwrap().to_owned();
            handle_attribute(
                &base_path,
                module_source_path,
                filename,
                attribute_name,
                attribute,
            )
        })
        .collect();

    let mut seen_attribute_keys = vec![];
    for attrlist in attribute_keys {
        for key in attrlist {
            if !seen_attribute_keys.contains(&key) {
                seen_attribute_keys.push(key);
            }
        }
    }
    // info!("Seen attribute keys:");
    seen_attribute_keys
        .iter()
        .for_each(|k| info!("attrkey {k}"));

    // check that the schema's all done
    let handled_fields: Vec<String> = [
        "description",
        "profiles",
        "name",
        "category",
        "caption", // TODO: do we care about the caption?
        "attributes",
    ]
    .into_iter()
    .map(|f| f.to_string())
    .collect();

    file_object.keys().for_each(|k| {
        if !handled_fields.contains(k) {
            warn!("Unhandled field in event schema: {k}");
        }
    });

    debug!("Description: {}", description);
    debug!("Profiles: {}", profiles.join(","));

    Ok(EventDef {
        uid: None, // TODO: uid?
        name,

        category,
        attribute_keys: seen_attribute_keys,
    })
}
