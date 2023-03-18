use crate::*;

// TODO: objects from the objects dir

pub fn generate_objects(paths: &DirPaths) -> Result<(), Box<dyn Error>> {
    let content = "//* hello world";
    write_source_file(
        &format!("{}src/objects.rs", paths.destination_path),
        content,
    )
}
