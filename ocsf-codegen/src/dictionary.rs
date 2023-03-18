use std::fmt::Debug;

use super::*;
use codegen::{Field, Variant};
// use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictAttribute {
    caption: String,
    default: Option<i32>,
    description: String,
    // #[serde(alias="enum")]
    attr_enum: Option<String>,
    is_array: Option<bool>,
    sibling: Option<String>,
    #[serde(alias = "type")]
    attr_type: TypeNames,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictType {
    caption: String,
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

    output.push_str(&format!("//* {}\n\n", dict_file.get("description").unwrap()));
    let mut output_scope = codegen::Scope::new();

    output_scope.raw("use serde::{Serialize};");

    output_scope.new_struct("DictAttribute")
        .vis("pub")
        .derive("Debug,Clone,Serialize")
        .push_field(Field::new("caption", "&'static str").vis("pub").to_owned())
        .push_field(Field::new("default", "Option<i32>").vis("pub").to_owned())
        .push_field(Field::new("description", "&'static str").vis("pub").to_owned())
        .push_field(Field::new("attr_enum", "Option<&'static str>").vis("pub").to_owned())
        .push_field(Field::new("is_array", "Option<bool>").vis("pub").to_owned())
        .push_field(Field::new("sibling", "Option<&'static str>").vis("pub").to_owned())
        .push_field(Field::new("attr_type", "TypeNames").vis("pub").annotation("#[serde(alias=\"type\")]").to_owned());

    output_scope.new_enum("TypeNames")
        .vis("pub")
        .derive("Debug,Clone,Serialize")
        .push_variant(Variant::new("Integer"))
        .push_variant(Variant::new("Json"))
        .push_variant(Variant::new("String"))
        .push_variant(Variant::new("Timestamp"))
        .push_variant(Variant::new("NotSupported{ name: &'static str }"));



    output_scope.new_struct("DictType")
        .vis("pub")
        .derive("Debug,Clone,Serialize")
        .push_field(Field::new("caption", "&'static str").vis("pub").to_owned())
        .push_field(Field::new("description", "Option<&'static str>").vis("pub").to_owned())
        .push_field(Field::new("max_len", "Option<&'static str>").vis("pub").to_owned())
        .push_field(Field::new("observable", "Option<&'static str>").vis("pub").to_owned())
        .push_field(Field::new("range", "Option<&'static str>").vis("pub").to_owned())
        .push_field(Field::new("regex", "Option<&'static str>").vis("pub").to_owned())
        .push_field(Field::new("value_type", "Option<&'static str>").vis("pub").to_owned())
        .push_field(Field::new("type_name", "Option<&'static str>").vis("pub").to_owned())
        .push_field(Field::new("values", "Option<&'static str>").vis("pub").to_owned())
        ;

    output.push_str(&output_scope.to_string());

    // let make_it_pub_re = Regex::new(r#"(?m)^(?P<thespace>\s+)(?P<theitem>\S+: (Some|None))"#).unwrap();

    dict_file
        .get("attributes")
        .unwrap()
        .as_object()
        .unwrap()
        .into_iter()
        .for_each(|(attribute_name, attribute_value)| {
            debug!("attribute_value: {attribute_value:#?}");
            let attribute: DictAttribute =
                serde_json::from_value(attribute_value.to_owned()).unwrap();
            debug!("{attribute:#?}");
            #[allow(clippy::single_char_add_str)]
            output.push_str("\n");
            let thing_to_push = format!(
                "pub const {}: DictAttribute = {:#?};\n",
                attribute_name.to_uppercase(),
                attribute
            );
            output.push_str(&thing_to_push);
        });

    Ok(output)
}

#[derive(Clone, Serialize)]
enum TypeNames {
    Integer,
    Json,
    String,
    Timestamp,
    NotSupported { name: String },
}

impl Debug for TypeNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "TypeNames::Integer"),
            Self::Json => write!(f, "TypeNames::Json"),
            Self::String => write!(f, "TypeNames::String"),
            Self::Timestamp => write!(f, "TypeNames::Timestamp"),
            Self::NotSupported { name } => {
                write!(f, "TypeNames::NotSupported{{ name: \"{name}\" }}")
                // .field("name", name)
                // .finish(),
            }
        }
    }
}

impl From<&str> for TypeNames {
    fn from(value: &str) -> Self {
        match value {
            "string_t" => Self::String,
            "integer_t" => Self::Integer,
            "json_t" => Self::Json,
            "timestamp_t" => Self::Timestamp,
            _ => Self::NotSupported {
                name: value.to_string(),
            },
        }
    }
}

impl<'de> Deserialize<'de> for TypeNames {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let stringval = String::deserialize(deserializer)?.to_lowercase();
        let result: TypeNames = stringval.into();
        Ok(result)
    }
}

impl From<String> for TypeNames {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<serde_json::Value> for TypeNames {
    fn from(value: serde_json::Value) -> Self {
        match value {
            Value::String(val) => val.as_str().into(),
            _ => panic!("Unsupported value type: {value:?}"),
        }
    }
}
