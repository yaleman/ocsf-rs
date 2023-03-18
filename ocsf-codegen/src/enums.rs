use crate::*;
use codegen::Variant;

pub fn add_enum(
    base_path: &str,
    module_source_path: &str,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let file_object = read_file_to_value(filename).unwrap();
    if !file_object.is_object() {
        error!("Not sure what this is!");
        error!("{:?}", file_object);
        panic!();
    }
    info!("Adding enum from {} to {}", filename, base_path);
    let enum_name = collapsed_title_case(base_path);

    let mut scope = Scope::new();
    // scope.new_enum(base_path);
    let mut scoped_enum = codegen::Enum::new(&enum_name)
        .derive("Debug")
        .vis("pub")
        .to_owned();

    let mut from_u8_impl = format!(
        "impl From<u8> for {enum_name} {{
    fn from(input: u8) -> Self {{
        match input {{\n"
    );

    let mut from_enum_impl = format!(
        "impl Into<u8> for {enum_name} {{
    fn into(self) -> u8 {{
        match &self {{\n"
    );

    let enum_object = file_object.as_object().unwrap().get("enum").unwrap();

    enum_object.as_object().unwrap().iter().for_each(|(k, v)| {
        // debug!("{k:?} => {v:?}");
        if !v.as_object().unwrap().contains_key("caption") {
            panic!();
        }
        let variant_name = v
            .as_object()
            .unwrap()
            .get("caption")
            .unwrap()
            .as_str()
            .unwrap();
        let this_variant = Variant::new(collapsed_title_case(variant_name))
            .annotation(format!("/// {k} - {variant_name}"))
            .to_owned();
        scoped_enum.push_variant(this_variant);
        from_u8_impl += &format!(
            "            {} => {}::{},\n",
            k,
            enum_name,
            collapsed_title_case(variant_name),
        );
        from_enum_impl += &format!(
            "            {}::{} => {},\n",
            enum_name,
            collapsed_title_case(variant_name),
            k,
        );
    });

    scope.push_enum(scoped_enum);

    let from_u8_impl = format!(
        "{from_u8_impl}            _ => panic!(\"Invalid value!\"),
        }}
    }}
}}
"
    );

    let from_enum_impl = format!(
        "{from_enum_impl}        }}
    }}
}}
"
    );

    let results = format!(
        "{}\n{}\n{}",
        scope.to_string(),
        from_u8_impl,
        from_enum_impl
    );
    write_source_file(
        &format!("{module_source_path}src/enums/{base_path}.rs"),
        &results,
    )?;
    Ok(())
}
