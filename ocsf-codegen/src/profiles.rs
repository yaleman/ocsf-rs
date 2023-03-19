use crate::*;

// TODO: profiles from the profiles dir

pub fn generate_profiles(paths: &DirPaths) -> Result<(), Box<dyn Error>> {
    let mut output_scope = codegen::Scope::new();

    output_scope.writeln("//* OCSF Profiles");
    output_scope.add_generation_timestamp_comment();

    write_source_file(
        &format!("{}src/profiles.rs", paths.destination_path),
        &output_scope.to_string(),
    )
}
