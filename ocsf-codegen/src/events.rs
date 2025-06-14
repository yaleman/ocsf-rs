use std::error::Error;

use codegen::{Field, Impl, Struct};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Map;
use serde_with::skip_serializing_none;

use crate::module::Module;
use crate::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct EventDef {
    pub uid: Option<u32>,
    pub class_name: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub attributes: HashMap<String, EventAttribute>,
    pub associations: Option<Map<String, Value>>,
    pub profiles: Option<Vec<String>>,
    extends: Option<String>,
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
            _ => panic!("Invalid enum value '{value}' - select from optional,recommended,required"),
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

// #[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[skip_serializing_none]
pub struct EventAttribute {
    name: Option<String>,
    profile: Option<String>,
    description: Option<String>,
    caption: Option<String>,
    requirement: Option<Requirement>,
    group: Option<Group>,
    #[serde(alias = "$include", skip)]
    include: Option<String>,
    /// This is the string name of the type, not the enum value
    enum_name: String,
    just_includes: Vec<String>,
}

impl<'de> Deserialize<'de> for EventAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        debug!("Deserializing to EventAttribute: {:?}", value);

        if value.is_array() {
            return Ok(EventAttribute {
                just_includes: value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect(),
                ..Default::default()
            });
        }

        let name = match value.get("name") {
            Some(n) => n
                .as_str()
                .map(|f| f.to_string().replace("type", "type_name")),
            None => None,
        };

        let profile = match value.get("profile") {
            Some(n) => n.as_str().map(|f| f.to_string()),
            None => None,
        };

        let description = match value.get("description") {
            Some(n) => n.as_str().map(|f| f.to_string()),
            None => None,
        };

        let caption = match value.get("caption") {
            Some(n) => n.as_str().map(|f| f.to_string()),
            None => None,
        };

        let requirement: Option<Requirement> = match value.get("requirement") {
            Some(n) => {
                let strval = n.as_str().unwrap();
                Some(Requirement::from(strval))
            }
            None => None,
        };

        let group: Option<Group> = match value.get("group") {
            Some(n) => {
                let strval = n.as_str().unwrap();
                Some(Group::from(strval))
            }
            None => None,
        };

        let includes: Vec<String> = match value.get("$include") {
            Some(include) => {
                let mut result: Vec<String> = vec![];

                if include.is_array() {
                    panic!(
                        "Found include array while deserializing an event attribute: {:?}",
                        include.as_array().unwrap()
                    );
                    // result.extend(include.as_array().unwrap().iter().map(|f| f.as_str().unwrap().to_string()));
                } else if include.is_string() {
                    trace!(
                        "Found include string or eventattribute: {:?}",
                        include.as_str().unwrap()
                    );
                    result.push(include.as_str().unwrap().to_string());
                } else {
                    panic!(
                        "found an include we can't handle in deserializing an attribute! {:?}",
                        include
                    );
                }
                result
            }
            None => vec![],
        };
        let mut enum_name = "String".to_string();
        if !includes.is_empty() {
            info!("Found includes: {:?}", includes);
            enum_name = format!(
                "crate::{}",
                collapsed_title_case(includes.first().unwrap().split('/').next_back().unwrap())
            );
        }

        Ok(EventAttribute {
            name,
            profile,
            description,
            caption,
            requirement,
            group,
            enum_name,
            ..Default::default()
        })
    }
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
            // because everything's a string at some point.:D how's
            enum_name: "String".to_string(),
            just_includes: vec![],
        }
    }
}

// impl EventAttribute {
//     pub fn name(self, name: String) -> Self {
//         Self {
//             name: Some(name),
//             ..self
//         }
//     }
//     pub fn profile(self, profile: Option<&str>) -> Self {
//         Self {
//             profile: profile.map(|val| val.to_string()),
//             ..self
//         }
//     }
//     pub fn description(self, description: String) -> Self {
//         Self {
//             description: Some(description),
//             ..self
//         }
//     }
//     pub fn caption(self, value: String) -> Self {
//         Self {
//             caption: Some(value),
//             ..self
//         }
//     }
//     pub fn requirement(self, value: Requirement) -> Self {
//         Self {
//             requirement: Some(value),
//             ..self
//         }
//     }
//     pub fn group(self, value: Group) -> Self {
//         Self {
//             group: Some(value),
//             ..self
//         }
//     }
// }

// /// returns an [EventAttribute] and  list of attribute names, so I can track down what I need to support :D
// fn handle_attribute(
//     _base_path: &str,
//     _module_source_path: &str,
//     filename: &str,
//     attribute_name: &str,
//     attribute: Map<String, Value>,
// ) -> EventAttribute {
//     let attrkeys: Vec<String> = attribute.keys().map(|k| k.to_string()).collect();
//     info!(
//         "Handling attribute {} (keys: {:#?})",
//         attribute_name,
//         attrkeys.join(",")
//     );

//     let mut result = EventAttribute::new(attribute_name.to_string());

//     attribute.iter().for_each(|(key, value)| {
//         info!("attr: {} val: {:?}", key, value);
//         result = match key.to_owned().as_str() {
//             "$include" => {
//                 // TODO: handle includes inside attributes!
//                 warn!(
//                     "Attribute {} in {} needs include: {}",
//                     attribute_name,
//                     filename,
//                     value.as_str().unwrap()
//                 );
//                 result.clone()
//             }
//             "caption" => result.clone().caption(value.as_str().unwrap().into()),
//             "description" => result.clone().description(value.as_str().unwrap().into()),
//             "enum" => {
//                 warn!(
//                     "Attribute {} in {} needs enum: {:?}",
//                     attribute_name,
//                     filename,
//                     value.to_string()
//                 );
//                 result.clone() // TODO: handle arbitrary enums inside of events!
//             }
//             "group" => result.clone().group(value.as_str().unwrap().into()),
//             "profile" => {
//                 debug!("profile value: {:?}", value.as_str());
//                 result.clone().profile(value.as_str())
//             }
//             "requirement" => result.clone().requirement(value.as_str().unwrap().into()),
//             _ => {
//                 warn!("Unhandled attr key: {key:?}");
//                 result.clone()
//             }
//         };
//     });
//     debug!("{result:#?}");
//     result
// }

// /// load a category name so we can extend from it
// fn load_base_module(paths: &DirPaths, category_name: String) -> Result<Value, String> {

//     let target_file = format!("{}events/{}/{}.json", paths.schema_path, category_name, category_name);

//     read_file_to_value(&target_file).map_err(|e| format!("{e:?}"))

// }

fn load_all_event_files(paths: &DirPaths) -> HashMap<String, EventDef> {
    let target_path = format!("{}events/", paths.schema_path);
    info!("loading all event files from {}", target_path);

    let mut result: HashMap<String, EventDef> = HashMap::new();

    for file in WalkDir::new(&target_path) {
        let file = match file {
            Ok(val) => val,
            Err(err) => {
                error!("Failed to walk dir somewhere: {err:?}");
                continue;
            }
        };
        if !file.clone().into_path().is_file() {
            debug!("Skipping {:?} as it's not a file...", file);
            continue;
        }
        debug!("Reading {file:?} into EventDef");
        let file_value =
            read_file_to_value(file.clone().into_path().as_os_str().to_str().unwrap()).unwrap();
        let file_event: EventDef = serde_json::from_value(file_value).unwrap();
        result.insert(
            file.into_path()
                .as_os_str()
                .to_str()
                .unwrap()
                .to_owned()
                .replace(&paths.schema_path, ""),
            file_event,
        );
    }
    result
}

pub fn generate_events(paths: &DirPaths, root_module: &mut Module) -> Result<(), Box<dyn Error>> {
    // let event_schema_path = format!("{}events/", paths.schema_path);
    // let filenames = find_files(&event_schema_path);

    let categories_file = read_file_to_value(&format!("{}categories.json", paths.schema_path))?;
    let categories_file = categories_file
        .get("attributes")
        .expect("Coudln't get categories file attributes");
    let categories: HashMap<String, Category> = serde_json::from_value(categories_file.to_owned())?;

    let mut all_events = load_all_event_files(paths);

    for (filename, event) in all_events.iter_mut() {
        if filename.len() <= 1 {
            warn!("Can't handle file {}", filename);
            continue;
        }

        let struct_name = event.name.to_owned();
        info!("Struct name: {} from {}", struct_name, filename);

        let target_module_path = PathBuf::from(filename.replace("events/", ""))
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        debug!("Putting it into module: {}", target_module_path);

        let mut target_module = root_module
            .children
            .get_mut("events")
            .expect("Couldn't get events module from root?");

        for tm in target_module_path.split('/') {
            if tm.is_empty() {
                continue;
            }
            if !target_module.has_child(tm) {
                target_module.add_child(tm.to_owned());
            }
            target_module = target_module
                .children
                .get_mut(tm)
                .unwrap_or_else(|| panic!("Couldn't get {tm}"));
        }

        debug!("Target module: {:#?}", target_module);

        // if !target_module.imports.iter().any(|e| e == "use serde::Deserialise") {
        //     target_module.imports.push("use serde::Deserialize".to_string());
        // }
        // if !target_module.imports.iter().any(|e| e == "use serde::Serialize") {
        //     target_module.imports.push("use serde::Serialize".to_string());
        // }

        // // if let Some(extends_category) = event.extends {
        // //     // info!("Loading {} as a base for {}", extends_category, filename);
        // //     // grab a copy of the base module
        // //     let base = load_base_module(paths, extends_category).unwrap();
        // //     let mut base = base.as_object().unwrap().to_owned();

        // //     // get a copy of the event so we can overlay the event def on the base
        // //     let orig_event = event_value.clone();
        // //     let orig_event = orig_event.as_object().unwrap();

        // //     orig_event.iter().for_each(|(key,value)| {
        // //         if key != "uid" {
        // //             base.insert(key.to_owned(), value.to_owned());
        // //         }
        // //     });

        let struct_doc = if let Some(description) = &event.description {
            format!("{}\n\nSourced from: `events/{}`", description, filename)
        } else {
            format!("Sourced from: `events/{}`", filename)
        };

        let mut module_struct = Struct::new(&collapsed_title_case(&event.name));
        module_struct
            .doc(&struct_doc)
            .vis("pub")
            .derive("serde::Deserialize")
            .derive("serde::Serialize");

        // yes, we're sorting struct fields.
        event
            .attributes
            .iter()
            .sorted_by(|a, b| Ord::cmp(a.0, b.0))
            .for_each(|(attr_name, attr)| {
                if attr_name == "$include" {
                    return error!("need to handle attribute $include {attr:#?}");
                    //TODO: need to parse attributes
                } else {
                    debug!("attr name: {attr_name}");
                }
                let attr_name = match attr_name == "type" {
                    true => "type_name",
                    false => attr_name,
                };

                let field_requirement_template: &'static str = match &attr.requirement {
                    Some(val) => match val {
                        Requirement::Optional => "Option<{}>",
                        Requirement::Recommended => "Option<{}>",
                        Requirement::Required => "{}",
                    },
                    None => "Option<{}>",
                };

                let mut attr_field = Field::new(
                    attr_name,
                    field_requirement_template.replace("{}", &attr.enum_name),
                );
                attr_field.vis("pub");
                // documentation is always nice
                if let Some(description) = &attr.description {
                    attr_field.doc(fix_docstring(description.to_owned(), None));
                }

                if attr_name == "type" {
                    // because when we serialize it out, it needs the right name
                    attr_field.annotation("#[serde(alias=\"type\")]");
                }

                module_struct.push_field(attr_field);
            });

        let mut module_impl = Impl::new(collapsed_title_case(&event.name));

        let mut uid = event.uid.unwrap_or(0);
        if let Some(category) = event.category.clone() {
            if categories.contains_key(&category) {
                uid += 1000 * categories[&category].uid;
                debug!("Set UID to {uid}");
            }
        }

        if uid != 0 {
            module_impl.associate_const("UID", "u16", format!("{}", uid), "pub");
        }

        target_module.scope.push_struct(module_struct);

        target_module.scope.push_impl(module_impl);
    }

    Ok(())
}
