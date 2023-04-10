//! Parse and handle `profiles/\*.json`
//!

use codegen::Struct;
use serde::{Deserialize, Serialize};

use crate::module::Module;
use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
/// Profile overrides for events
pub struct Profile {
    /// seems to note that it's ... a 'primary' profile?
    pub annotations: Option<HashMap<String, Value>>,
    pub attributes: HashMap<String, EventAttribute>,

    /// ... shorter name
    pub caption: String,

    /// long name for the thing
    pub description: String,

    // basically says "profile"... because it's a profile.
    // pub meta: String,
    /// identifier for the object
    pub name: String,
}

impl Profile {
    // return the filename for this profile
    pub fn filename(&self) -> String {
        format!("profiles/{}.json", self.name)
    }

    /// add this profile to a given codegen Scope
    pub fn add_to_scope(&self, scope: &mut Scope) -> Result<(), String> {

        let mut profile_struct = Struct::new(&self.name);

        let mut profile_docstring = String::new();
        if self.description.trim() != "" {
            profile_docstring.push_str(&self.description);
        }

        profile_struct.doc(&profile_docstring)
            .vis("pub")
            .derive("Debug")
            .derive("serde::Deserialize")
            .derive("serde::Serialize")
            ;

        profile_struct.field("name", &format!("String"));
        profile_struct.field("caption", &format!("String"));
        profile_struct.field("description", &format!("String"));
        profile_struct.field("annotations", &format!("Option<HashMap<String, Value>>"));
        profile_struct.field("attributes", &format!("HashMap<String, EventAttribute>"));


        scope.push_struct(profile_struct);
        Ok(())
    }
}


pub fn generate_profiles(
    paths: &DirPaths,
    root_module: &mut Module,
) -> Result<(), Box<dyn Error>> {

    for filename in WalkDir::new(format!("{}profiles", paths.schema_path)) {
        if let Ok(filename) = filename {
            if !filename.path().is_file() {
                continue;
            }
            let filename_str = filename.clone().into_path();
            let filename_str = filename_str.to_string_lossy().into_owned();

            let file_contents = read_file_to_value(&filename_str)?;
            debug!("Processing profile filename {filename:?}");

            let profile: Profile = serde_json::from_value(file_contents)?;

            root_module.profiles.insert(profile.name.clone(), profile.clone());

        } else {
            error!("Failed to handle {filename:?}");
        }
    }

    Ok(())
}
