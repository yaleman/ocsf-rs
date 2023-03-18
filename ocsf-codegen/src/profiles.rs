use crate::*;

// TODO: profiles from the profiles dir

pub fn generate_profiles(paths: &DirPaths) -> Result<(), Box<dyn Error>> {
    let content = "//* hello world";
    write_source_file(
        &format!("{}src/profiles.rs", paths.destination_path),
        content,
    )
}
