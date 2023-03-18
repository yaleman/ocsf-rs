use crate::*;

// TODO: categories from the categories file

pub fn generate_categories(paths: &DirPaths) -> Result<(), Box<dyn Error>> {
    let content = "//* hello world";
    write_source_file(
        &format!("{}src/categories.rs", paths.destination_path),
        content,
    )
}
