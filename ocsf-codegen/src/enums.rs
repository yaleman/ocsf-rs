use crate::*;
use codegen::{Enum, Function, Variant};
use serde::{Deserialize, Serialize};
use itertools::Itertools;

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
//         enum_tryfrom_u8.line(&format!("{} => {}::{},",
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumData {
    caption: String,
    description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EnumFile {
    #[serde(alias = "enum")]
    elements: HashMap<u8, EnumData>,
}

pub fn get_enum_defaults(paths: &DirPaths) -> Result<HashMap<u8, EnumData>, Box<dyn Error>> {
    let defaults = format!("{}enums/defaults.json", paths.schema_path);
    trace!("Pulling defaults from {defaults}");
    let defaults: EnumFile = serde_json::from_value(read_file_to_value(&defaults)?)?;
    let defaults = defaults.elements;

    trace!("Defaults: {defaults:#?}");
    Ok(defaults)
}

pub fn generate_enums(paths: &DirPaths) -> Result<(), Box<dyn Error>> {
    let mut output_scope = Scope::new();

    let defaults = get_enum_defaults(paths)?;

    for filename in find_files(&format!("{}enums", paths.schema_path)) {
        debug!("Enum filename: {filename}");

        let enum_file = read_file_to_value(&filename)?;
        // debug!("{parsed_file:#?}");
        enum_from_value(
            paths,
            &mut output_scope,
            enum_file,
            collapsed_title_case(filename.split('/').last().unwrap()),
            Some(defaults.clone()),
        )?;
    }

    debug!("{}", output_scope.to_string());
    write_source_file(
        &format!("{}src/enums/mod.rs", paths.destination_path),
        &output_scope.to_string(),
    )
}

/// the passed output scope gets the shiny new enum, grats to the scope!
pub fn enum_from_value(
    paths: &DirPaths,
    output_scope: &mut Scope,
    value: Value,
    enum_name: String,
    defaults: Option<HashMap<u8, EnumData>>,
) -> Result<(), Box<dyn Error>> {
    let mut base_object = defaults.unwrap_or(get_enum_defaults(&paths)?);

    let parsed_file: EnumFile = serde_json::from_value(value)?;

    parsed_file.elements.into_iter().for_each(|(key, value)| {
        base_object.insert(key, value);
    });
    debug!("{base_object:#?}");

    let mut new_enum = Enum::new(&enum_name);
    new_enum.vis("pub");

    let mut enum_to_u8 = Function::new("from");
    enum_to_u8.arg("input", &enum_name).ret("u8");
    enum_to_u8.line("match input {");

    let mut try_u8_to_enum = Function::new("try_from");
    try_u8_to_enum
        .arg("input", "u8")
        .ret("Result<Self, String>");
    try_u8_to_enum.line("let res = match input {");

    base_object.keys().sorted().for_each(|key| {
        let value = base_object.get(&key).unwrap();
        let variant_name = collapsed_title_case(&value.caption);
        let mut variant = Variant::new(&variant_name);
        if let Some(description) = &value.description {
            variant.annotation(&format!("/// {}", description));
        }
        new_enum.push_variant(variant);

        enum_to_u8.line(&format!("    {}::{} => {},", enum_name, variant_name, &key));
        try_u8_to_enum.line(&format!("    {} => {}::{},", &key, enum_name, variant_name));
    });


    debug!("Adding enum called {enum_name} to scope...");
    enum_to_u8.line("}");
    try_u8_to_enum.line("_ => return Err(\"invalid value\".to_string()),");
    try_u8_to_enum.line("}; Ok(res)");
    // debug!("{:#?}",enum_to_u8.to_owned());

    output_scope.push_enum(new_enum);

    let enum_to_u8_impl = output_scope.new_impl("u8");
    enum_to_u8_impl.impl_trait(&format!("From<{enum_name}>"));
    enum_to_u8_impl.push_fn(enum_to_u8);

    let try_u8_to_enum_impl = output_scope.new_impl(&enum_name);
    try_u8_to_enum_impl
        .impl_trait("TryFrom<u8>")
        .associate_type("Error", "String");
    try_u8_to_enum_impl.push_fn(try_u8_to_enum);

    Ok(())
}
