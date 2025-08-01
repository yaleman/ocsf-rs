use serde::Deserialize;

use crate::*;

#[derive(Deserialize)]
struct VersionFile {
    version: String,
}

pub fn get_schema_version(paths: &DirPaths) -> Result<String, Box<dyn Error>> {
    let version_file = read_file_to_value(&format!("{}version.json", paths.schema_path))?;

    let schema_version: VersionFile = serde_json::from_value(version_file)?;
    Ok(schema_version.version)
}

pub fn add_version_element(
    paths: &DirPaths,
    output_scope: &mut codegen::Scope,
) -> Result<(), Box<dyn Error>> {
    let schema_version = get_schema_version(paths)?;

    debug!("OCSF Schema Version: {}", schema_version);
    output_scope.raw(format!(
        "// The schema version that the code was generated from ({}).",
        &schema_version
    ));
    output_scope.raw(format!(
        "pub static OCSF_SCHEMA_VERSION: &str = {:?};\n\n",
        schema_version
    ));

    Ok(())
}

// pub fn generate_other(paths: &DirPaths) -> Result<(), Box<dyn Error>> {
//     let mut output_scope = codegen::Scope::new();

//     output_scope.writeln(
//         "//* I'm not saying it's the junk drawer, but it's also *not* *not* the junk drawer.",
//     );
//     output_scope.add_generation_timestamp_comment();

//     add_version_element(paths, &mut output_scope)?;

//     write_source_file(
//         &format!("{}src/other.rs", paths.destination_path),
//         &output_scope.to_string(),
//     )
// }
