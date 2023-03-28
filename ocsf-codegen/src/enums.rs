use crate::module::{Module, ModuleEnumWithU8};
use crate::*;
// use codegen::{Enum, Function, Variant};
use serde::{Deserialize, Serialize};
// use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug)]
pub struct EnumDef {
    class_name: String,
}

#[derive(Debug, Clone,PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumData {
    pub caption: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumFile {
    #[serde(alias = "enum")]
    pub elements: HashMap<u8, EnumData>,
}

pub fn generate_enums(paths: &DirPaths, root_module: &mut Module) -> Result<(), Box<dyn Error>> {
    // let mut output_scope = Scope::new();

    // let defaults = get_enum_defaults(paths)?;

    for filename in find_files(&format!("{}enums", paths.schema_path)) {
        debug!("Enum filename: {filename}");

        let enum_file = read_file_to_value(&filename)?;
        // debug!("{parsed_file:#?}");
        let enum_data: crate::module::ModuleEnumWithU8 = enum_from_value(
            paths,
            root_module,
            enum_file,
            collapsed_title_case(filename.split('/').last().unwrap()),
        )?;

        root_module.enums.push(enum_data);
    }
    Ok(())
}

impl std::fmt::Display for OcsfCodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OcsfCodegenError: {}", self.errortext)
    }
}

/// Insert a new enum into the target module's scope
pub fn enum_from_value(
    paths: &DirPaths,
    root_module: &mut Module,
    value: Value,
    name: String,
) -> Result<ModuleEnumWithU8, OcsfCodegenError> {
    if root_module.has_enum(&name) {
        return Err(OcsfCodegenError::new(format!("Already has enum {name}!")));
    }

    let mut base_object = ModuleEnumWithU8::new(paths, name);

    let parsed_file: EnumFile = serde_json::from_value(value)
        .map_err(OcsfCodegenError::from)
        .unwrap();

    parsed_file.elements.into_iter().for_each(|(key, value)| {
        base_object.variants.insert(key, value);
    });
    debug!("{base_object:#?}");

    // root_module.enums.push(base_object);
    Ok(base_object)
}
