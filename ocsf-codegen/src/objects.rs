use crate::*;

// TODO: objects from the objects dir

pub fn generate_objects(paths: &DirPaths) -> Result<(), Box<dyn Error>> {
    let mut output_scope = codegen::Scope::new();

    output_scope.writeln("//* hello world");
    write_source_file(
        &format!("{}src/objects.rs", paths.destination_path),
        &output_scope.to_string(),
    )
}
