//! Parse and handle `profiles/\*.json`
//!

use crate::module::Module;
use crate::*;

// TODO: profiles from the profiles dir

pub fn generate_profiles(paths: &DirPaths, root_module: &mut Module) -> Result<(), Box<dyn Error>> {
    // let mut output_scope = codegen::Scope::new();

    let profiles_module = root_module
        .children
        .get_mut("profiles")
        .expect("Couldn't find the profiles module in root?");

    profiles_module.scope.writeln("//* OCSF Profiles");
    profiles_module.scope.add_generation_timestamp_comment();

    write_source_file(
        &format!("{}src/profiles.rs", paths.destination_path),
        &profiles_module.scope.to_string(),
    )
}
