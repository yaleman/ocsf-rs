use crate::*;
use crate::module::Module;
use codegen::{Function, Variant};
use serde::Deserialize;

// TODO: categories from the categories file

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Category {
    caption: String,
    description: String,
    uid: u8,
}

pub fn generate_categories(paths: &DirPaths, root_module: &mut Module) -> Result<(), Box<dyn Error>> {

    let categories_module = root_module.children.get_mut("categories").expect("Couldn't find categories module in root module?");


    let categories_file = read_file_to_value(&format!("{}categories.json", paths.schema_path))?;

    let categories_description: String = categories_file
        .get("description")
        .unwrap_or(&Value::String("".to_string()))
        .as_str()
        .unwrap()
        .to_string();

    categories_module.scope.writeln("//! OCSF Category data");
    categories_module.scope.writeln("//!".to_string());
    categories_module.scope.writeln(format!("//! {categories_description}"));

    categories_module.scope.add_generation_timestamp_comment();

    let enum_name = "Category";

    // enum Category
    let mut category_enum = codegen::Enum::new(enum_name);
    category_enum.vis("pub").doc(&categories_description);

    // impl From<Category> for u8
    let mut category_to_u8 = Function::new("from");
    category_to_u8.arg("input", enum_name).ret("u8");
    category_to_u8.line("match input {");

    // impl TryFrom<u8> for Category
    let mut u8_to_category = Function::new("try_from");
    u8_to_category
        .arg("input", "u8")
        .ret("Result<Self, String>");
    // u8_to_category.line("type Error = String;");
    u8_to_category.line("let res = match input {");

    // generate details based on the attributes
    if let Some(attributes) = categories_file.get("attributes") {
        if let Some(attributes_object) = attributes.as_object() {
            attributes_object.into_iter().for_each(|(key, value)| {
                let category: Category = serde_json::from_value(value.clone()).unwrap();
                // debug!("{key} {category:#?}");
                let variant_name = collapsed_title_case(key);

                let variant_docstring = vec![
                    format!("/// {}", category.description),
                    "///".to_string(),
                    format!("/// `uid={}`", category.uid),
                ];

                let variant = Variant::new(&variant_name)
                    .annotation(variant_docstring.join("\n"))
                    .to_owned();
                category_enum.push_variant(variant);
                category_to_u8.line(format!(
                    "{}::{} => {},",
                    enum_name, variant_name, category.uid
                ));
                u8_to_category.line(format!(
                    "{} => {}::{},",
                    category.uid, enum_name, variant_name
                ));
            })
        }
    }

    // finish up From<Category> for u8
    category_to_u8.line("}");
    let mut category_to_u8_impl = codegen::Impl::new("u8");
    category_to_u8_impl.impl_trait(format!("From<{enum_name}>"));
    category_to_u8_impl.push_fn(category_to_u8);

    // finish up TryFrom<u8> for Category
    u8_to_category.line("_ => return Err(format!(\"Invalid value specified: {input}\")),\n");
    u8_to_category.line("};");
    u8_to_category.line("Ok(res)");
    let mut u8_to_category_impl = codegen::Impl::new(enum_name);
    u8_to_category_impl
        .impl_trait("TryFrom<u8>")
        .associate_type("Error", "String");
    u8_to_category_impl.push_fn(u8_to_category);

    categories_module.scope.push_enum(category_enum.to_owned());
    categories_module.scope.push_impl(category_to_u8_impl.to_owned());
    categories_module.scope.push_impl(u8_to_category_impl.to_owned());

    write_source_file(
        &format!("{}src/categories.rs", paths.destination_path),
        &categories_module.scope.to_string(),
    )
}
