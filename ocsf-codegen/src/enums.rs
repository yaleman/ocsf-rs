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

// pub fn add_enum(
//     base_path: &str,
//     module_source_path: &str,
//     filename: &str,
// ) -> Result<(), Box<dyn Error>> {
//     let file_object = read_file_to_value(filename).unwrap();
//     if !file_object.is_object() {
//         error!("Not sure what this is!");
//         error!("{:?}", file_object);
//         panic!();
//     }
//     info!("Adding enum from {} to {}", filename, base_path);
//     let enum_name = collapsed_title_case(base_path);

//     let mut scope = Scope::new();
//     // scope.new_enum(base_path);
//     let mut scoped_enum = codegen::Enum::new(&enum_name)
//         .derive("Debug")
//         .vis("pub")
//         .to_owned();

//     // impl TryFrom<u8> for Category
//     let mut enum_tryfrom_u8 = Function::new("try_from");
//     enum_tryfrom_u8.arg("input", "u8")
//         .ret("Result<Self, String>")
//         ;
//     // u8_to_category.line("type Error = String;");
//     enum_tryfrom_u8.line("let res = match input {");

//     let mut from_enum_impl = format!(
//         "impl Into<u8> for {enum_name} {{
//     fn into(self) -> u8 {{
//         match &self {{\n"
//     );

//     let enum_object = file_object.as_object().unwrap().get("enum").unwrap();

//     enum_object.as_object().unwrap().iter().for_each(|(k, v)| {
//         // debug!("{k:?} => {v:?}");
//         if !v.as_object().unwrap().contains_key("caption") {
//             panic!();
//         }
//         let variant_name = v
//             .as_object()
//             .unwrap()
//             .get("caption")
//             .unwrap()
//             .as_str()
//             .unwrap();
//         let this_variant = Variant::new(collapsed_title_case(variant_name))
//             .annotation(format!("/// {k} - {variant_name}"))
//             .to_owned();
//         scoped_enum.push_variant(this_variant);
//         enum_tryfrom_u8.line(format!("{} => {}::{},",
//             k,
//             enum_name,
//             collapsed_title_case(variant_name),
//         ));
//         from_enum_impl += &format!(
//             "            {}::{} => {},\n",
//             enum_name,
//             collapsed_title_case(variant_name),
//             k,
//         );
//     });

//     scope.push_enum(scoped_enum);

//     let enum_tryfrom_u8_impl = scope.new_impl(&enum_name);
//     enum_tryfrom_u8_impl.associate_type("Error", "String");

// //     let from_u8_impl = format!(
// //         "{from_u8_impl}            _ => panic!(\"Invalid value!\"),
// //         }}
// //     }}
// // }}
// // "
// //     );

//     let from_enum_impl = format!(
//         "{from_enum_impl}        }}
//     }}
// }}
// "
//     );

//     let results = format!(
//         "{}\n{}",
//         scope.to_string(),
//         // from_u8_impl,
//         from_enum_impl
//     );
//     write_source_file(
//         &format!("{module_source_path}src/enums/{base_path}.rs"),
//         &results,
//     )?;
//     Ok(())
// }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
