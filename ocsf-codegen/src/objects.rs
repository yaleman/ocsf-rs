use serde::{Deserialize, Serialize};

use crate::module::Module;
use crate::*;

// TODO: objects from the objects dir

#[derive(Debug, Serialize, Deserialize)]
struct ObjectDef {
    caption: String,
    description: String,
    name: String,
    extends: Option<String>,
    // #[serde(skip)]
    attributes: HashMap<String, Value>,

    constraints: Option<HashMap<String, Vec<String>>>,
}

pub fn generate_objects(paths: &DirPaths, root_module: &mut Module) -> Result<(), Box<dyn Error>> {
    // let mut output_scope = codegen::Scope::new();
    let object_module = root_module
        .children
        .get_mut("objects")
        .expect("Couldn't get objects module from root?");

    object_module.scope.writeln("//* hello world");

    for filename in find_files(&format!("{}objects/", &paths.schema_path)) {
        debug!("Object file: {filename:?}");

        let file_value = read_file_to_value(&filename)?;
        let objectdef: ObjectDef = serde_json::from_value(file_value)?;

        trace!("Object Def: {objectdef:#?}");
    }

    write_source_file(
        &format!("{}src/objects.rs", paths.destination_path),
        &object_module.scope.to_string(),
    )
}
