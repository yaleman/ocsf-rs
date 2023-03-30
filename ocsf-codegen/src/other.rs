//! It's not the junk drawer, I swear.
//!

use crate::*;
use serde::Deserialize;

#[derive(Deserialize)]
/// Deserialization target for `version.json`
struct VersionFile {
    /// Version value
    version: String,
}

/// Add a version tag to a given `codegen::Scope` object, to tag files with build/commit data.
pub fn add_version_element(
    paths: &DirPaths,
    output_scope: &mut codegen::Scope,
) -> Result<(), Box<dyn Error>> {
    let version_file = read_file_to_value(&format!("{}version.json", paths.schema_path))?;

    let schema_version: VersionFile = serde_json::from_value(version_file)?;
    debug!("OCSF Schema Version: {}", schema_version.version);
    output_scope.raw(format!(
        "/// This was the schema version that the code was generated from ({}).",
        &schema_version.version
    ));
    output_scope.raw(format!(
        "pub static OCSF_SCHEMA_VERSION: &str = {:?};\n\n",
        schema_version.version
    ));

    Ok(())
}
