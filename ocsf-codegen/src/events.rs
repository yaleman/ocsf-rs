//! Parses the events/\*.json file data, results go into `ocsf::events`.
//!

use std::error::Error;

use codegen::{Field, Function, Impl, Struct};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Map;

use crate::module::Module;
use crate::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
/// Deserialization target for `events/\*.json` files.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    extends: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
/// Deserialization elper
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
/// Deserialization helper for the "requirement" field, used all over the place.
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

        trace!("Deserializing to EventAttribute: {:?}", value);

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
                collapsed_title_case(includes.first().unwrap().split('/').last().unwrap())
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
        let mut file_event: EventDef = serde_json::from_value(file_value).unwrap();
        // stripping out the include value, because by this point we should have handled it!
        file_event.attributes.remove("$include");

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

/// Generates the events structs. Here be 🐉
pub fn generate_events(paths: &DirPaths, root_module: &mut Module) -> Result<(), Box<dyn Error>> {
    let categories_file = read_file_to_value(&format!("{}categories.json", paths.schema_path))?;
    let categories_file = categories_file
        .get("attributes")
        .expect("Coudln't get categories file attributes");

    let profiles = generate_profiles(paths, root_module)?;
    let categories: HashMap<String, Category> = serde_json::from_value(categories_file.to_owned())?;

    let mut all_events = load_all_event_files(paths);

    for (filename, event) in all_events
        .iter_mut()
        .sorted_by_key(|(_k, v)| v.name.clone())
    {
        if let Some(event_profiles) = event.profiles.clone() {
            event_profiles
                .into_iter()
                .sorted()
                .for_each(|profile_name| {
                    // take the profile name and get it from the pre-loaded profiles
                    let event_profile = profiles
                        .get(&profile_name)
                        .unwrap_or_else(|| panic!("Can't find {profile_name} in profiles"));

                    // TODO: #11 - work out what profile annotations do?
                    if let Some(annotations) = event_profile.annotations.clone() {
                        annotations.iter().for_each(|(annot_name, annot)| {
                            debug!("Annotation {annot_name} -> {annot:?}");
                        });
                    }

                    // process the attributes
                    event_profile
                        .attributes
                        .iter()
                        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
                        .for_each(|(attr_name, attr)| {
                            // trace!("Attribute {attr_name} -> {attr:?}");
                            if event.attributes.contains_key(attr_name.as_str()) {
                                debug!("new attr: {attr:?}");
                                debug!("existing: {:?}", event.attributes.get(attr_name.as_str()));
                                error!("duplicate attribute name: {attr_name}")
                            } else {
                                event
                                    .attributes
                                    .insert(attr_name.to_owned(), attr.to_owned());
                            }
                        });
                })
        }

        let struct_name = event.name.to_owned();
        info!("Struct name: {} from {}", struct_name, filename);

        let target_module_path = PathBuf::from(filename.replace("events/", ""))
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        trace!("Putting struct '{struct_name}' into module: {target_module_path}",);

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

        trace!("Target module: {:#?}", target_module);

        let struct_doc = format!("{}\n\nSourced from: `{}`", &event.description, filename);
        let mut module_struct = Struct::new(&collapsed_title_case(&event.name));
        module_struct
            .doc(&struct_doc)
            .vis("pub")
            .derive("serde::Deserialize")
            .derive("serde::Serialize")
            .derive("Default")
            .derive("Debug");

        let mut func_new = Function::new("new");
        func_new
            .vis("pub")
            .ret("Self")
            .doc("Create a new instance of this event.");

        func_new.line("Self {");

        let mut module_impl = Impl::new(&collapsed_title_case(&event.name));

        // yes, we're sorting struct fields.
        // here we handle all the attributes...
        event
            .attributes
            .iter()
            .sorted_by(|a, b| Ord::cmp(a.0, b.0))
            .for_each(|(attr_name, attr)| {
                if attr_name == "$include" {
                    return error!(
                        "need to handle attribute $include {:#?}",
                        attr.just_includes
                    );
                    //TODO: need to handle  attribute includes
                } else {
                    trace!("attr name: {attr_name}");
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
                let mut attr_docstring = String::new();
                if let Some(description) = &attr.description {
                    attr_docstring += &fix_docstring(description.to_owned(), None);
                } else if let Some(caption) = &attr.caption {
                    attr_docstring += &fix_docstring(caption.to_owned(), None);
                } else {
                    attr_docstring += "No description available."
                }

                if let Some(requirement) = attr.requirement.clone() {
                    let req: &str = requirement.into();
                    attr_docstring += &format!(" - {}", req);
                }
                let mut serde_annotations: Vec<&str> = vec![];
                if attr_name == "type_name" {
                    // because when we serialize it out, it needs the right name
                    serde_annotations.push("alias=\"type\"");
                }
                func_new.doc(attr_docstring);
                // add the attributes to the new() function
                if attr.requirement.is_some() && attr.requirement == Some(Requirement::Required) {
                    func_new.arg(attr_name, &attr.enum_name);
                    func_new.line(format!("{attr_name},"));
                } else {
                    func_new.line(format!("{attr_name}: None,"));
                    let mut with_func = Function::new(format!("with_{attr_name}"));

                    with_func
                        .vis("pub")
                        .doc(format!("Set the value of {}", attr_name))
                        .arg_self()
                        .arg(attr_name, &attr.enum_name)
                        .ret("Self");

                    with_func.line(format!("Self {{ {attr_name}: Some({attr_name}),"));
                    if event.attributes.len() > 1 {
                        with_func.line("..self  ");
                    }
                    with_func.line("}");

                    module_impl.push_fn(with_func);

                    // if it's optional, then we need to tell serde to ignore it on serialization
                    serde_annotations.push("skip_serializing_if = \"Option::is_none\"");
                }

                if !serde_annotations.is_empty() {
                    attr_field.annotation(&format!("#[serde({})]", serde_annotations.join(",")));
                }

                // add builders for the not-required fields

                module_struct.push_field(attr_field);
            });

        // this is the end of the Self::new() function
        func_new.line("}");

        let mut uid = event.uid.unwrap_or(0);
        if let Some(category) = event.category.clone() {
            if categories.contains_key(&category) {
                uid += 1000 * categories[&category].uid;
                trace!("Set UID to {uid}");
            }
        }
        if uid != 0 {
            module_impl.associate_const("UID", "u16", format!("{}", uid), "pub");
        }

        module_impl.push_fn(func_new);

        target_module.scope.push_struct(module_struct);
        target_module.scope.push_impl(module_impl);
    }

    Ok(())
}
