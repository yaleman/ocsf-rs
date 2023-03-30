//! Parse and handle `profiles/\*.json`
//!

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
    pub fn filename(&self) -> String {
        format!("profiles/{}.json", self.name)
    }
}

pub fn generate_profiles(
    paths: &DirPaths,
    _root_module: &mut Module,
) -> Result<HashMap<String, Profile>, Box<dyn Error>> {
    let mut results: HashMap<String, Profile> = HashMap::new();

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
            results.insert(profile.name.clone(), profile.to_owned());

            trace!(
                "comp {}, {}, {}",
                &profile.name,
                filename_str.replace(&paths.schema_path, ""),
                profile.filename(),
            );
        } else {
            error!("Failed to handle {filename:?}");
        }
    }

    Ok(results)
}
