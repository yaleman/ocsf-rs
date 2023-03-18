use super::*;
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictAttribute {
    caption: Option<String>,
    default: Option<i32>,
    description: Option<String>,
    attr_enum: Option<String>,
    is_array: Option<bool>,
    sibling: Option<String>,
    attr_type: Option<String>,
}


#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictType {
    caption: Option<String>,
    description: Option<String>,
    max_len: Option<String>,
    observable: Option<String>,
    range: Option<String>,
    regex: Option<String>,
    value_type: Option<String>,
    type_name: Option<String>,
    values: Option<String>,
}

pub fn parse_dictionary_file(paths: &DirPaths) -> Result<String, Box<dyn Error>> {

    let dict_filepath = format!("{}/dictionary.json", paths.schema_path);

    let dict_file = read_file_to_value(&dict_filepath)?;

    // debug!("{dict_file:#?}");

    let mut output = String::new();

    output.push_str(&format!("//* {}", dict_file.get("description").unwrap()));
    output.push_str("
use serde::{Serialize, Deserialize};


#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DictAttribute {
    caption: Option<String>,
    default: Option<i32>,
    description: Option<String>,
    attr_enum: Option<String>,
    is_array: Option<bool>,
    sibling: Option<String>,
    attr_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DictType {
    caption: Option<String>,
    description: Option<String>,
    max_len: Option<String>,
    observable: Option<String>,
    range: Option<String>,
    regex: Option<String>,
    value_type: Option<String>,
    type_name: Option<String>,
    values: Option<String>,
}");

    dict_file.get("attributes").unwrap()
        .as_object()
        .unwrap()
        .into_iter()
        .for_each(|(attribute_name, attribute_value)|
    {
        debug!("attribute_value: {attribute_value:#?}");
        let attribute: DictAttribute = serde_json::from_value(attribute_value.to_owned()).unwrap();
        debug!("{attribute:#?}");
        output.push_str("\n");
        let thing_to_push = format!(
            "lazy_static! {{ static ref {}: DictAttribute = {:#?};\n}}\n",
            attribute_name.to_uppercase(),
            attribute
        )
            .replace("\",\n", "\".to_string(),\n");
        output.push_str(&thing_to_push);
        // attribute
    });



    Ok(output)
}

enum TypeNames {
    String,
    Integer
}

impl From<serde_json::Value> for TypeNames {
    fn from(value: serde_json::Value) -> Self {
        match value {
            Value::String(val) => match val.as_str() {
                "string_t" => Self::String,
                "integer_t" => Self::Integer,
                _ => panic!("Unsupported type: {val:?}"),
            },
            _ => panic!("Unsupported value type: {value:?}"),
            // Value::Null => todo!(),
            // Value::Bool(_) => todo!(),
            // Value::Number(_) => todo!(),
            // Value::Array(_) => todo!(),
            // Value::Object(_) => todo!(),
        }
    }
}