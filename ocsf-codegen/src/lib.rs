use std::error::Error;
use std::fs::{File, create_dir_all};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

#[allow(unused_imports)]
use codegen::Scope;
use codegen::Variant;
use serde_json::{self, Value};
use walkdir::{DirEntry, WalkDir};

pub fn find_files(base_path: &str) -> Vec<String> {
    let schema_dir = format!("{base_path}ocsf-schema/");
    println!("looking for schema files in {schema_dir}");
    let files: Vec<DirEntry> = WalkDir::new(&schema_dir)
        .into_iter()
        .filter_map(|p| p.ok())
        .collect();
    files
        .iter()
        .filter_map(|p| match p.path().is_file() {
            true => Some(p.path().to_string_lossy().into()),
            false => None,
        })
        .collect()
}

#[derive(Debug)]
pub enum ClassPath {
    Enums { class_path: String },
    Unknown,
}
pub fn filename_to_classpath(schema_base_path: &str, filename: &str) -> ClassPath {
    eprintln!("Filename: {filename}");
    eprintln!("schema_base_path: {schema_base_path}");
    let mut fname = filename.to_owned().replace(schema_base_path, "");
    println!("This isn't the classpath, yet: {fname}");
    if fname.starts_with("enums/") {
        fname =  fname
                .replace("enums/", "")
                .replace(".json", "");
        return ClassPath::Enums {
            class_path: fname.to_string(),
        };
    } else {
        println!("Couldn't find enums/ in {fname}");
    }
    ClassPath::Unknown
    // filename.to_string()
}

fn collapsed_title_case(input: &str) -> String {
    let res = input.to_string();
    string_morph::to_title_case(
        &res
            .replace("enums/", "")
            .replace(".json", "")
            .replace("_", " "),
    )
    .replace(" ", "")
}

pub fn add_enum(base_path: &str, module_source_path: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let file_object = read_file_to_value(filename).unwrap();
    if !file_object.is_object() {
        eprintln!("Not sure what this is!");
        eprintln!("{:?}", file_object);
        panic!();
    }
    println!("Adding enum from {} to {}", filename, base_path);
    let enum_name = collapsed_title_case(&base_path);

    let mut scope = Scope::new();
    // scope.new_enum(base_path);
    let mut scoped_enum = codegen::Enum::new(&enum_name)
    .derive("Debug")
    .vis("pub")
    .to_owned();

    let mut from_u8_impl = format!("impl From<u8> for {enum_name} {{
    fn from(input: u8) -> Self {{
        match input {{\n");

    let enum_object = file_object.as_object().unwrap().get("enum").unwrap();

    enum_object.as_object().unwrap().iter().for_each(|(k, v)| {
        eprintln!("{k:?} => {v:?}");
        if !v.as_object().unwrap().contains_key("caption") {
            panic!();
        }
        let variant_name = v
            .as_object()
            .unwrap()
            .get("caption")
            .unwrap()
            .as_str()
            .unwrap();
        let this_variant = Variant::new(collapsed_title_case(variant_name))
            .annotation(format!("/// {k} - {variant_name}"))
            .to_owned();
        scoped_enum.push_variant(this_variant);
        from_u8_impl += &format!("            {} => {}::{},\n",
        k,
        enum_name,
        collapsed_title_case(variant_name));
    });

    scope.push_enum(scoped_enum);

    let from_u8_impl = format!("{from_u8_impl}            _ => panic!(\"Invalid value!\"),
        }}
    }}
}}
");

    let results = format!("{}\n{}", scope.to_string(), from_u8_impl);
    write_source_file(&format!("{module_source_path}src/enums/{base_path}.rs"), &results)?;
    Ok(())
}

pub fn write_source_file(filename: &str, contents: &str) -> Result<(), Box<dyn Error>>{
    // let mut path = PathBuf::from("");
    // path.set_file_name(filename);
    eprintln!("trying to write to {:?}", filename);
    // TODO maybe we will need to be appending to this at some point...
    let file = File::create(format!("{filename}"))?;
    let mut writer = BufWriter::new(file);
    writer.write(contents.as_bytes())?;
    println!("successfully wrote {filename}!");
    Ok(())
}

pub fn read_file_to_value(filename: &str) -> Result<Value, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let res: Value = serde_json::from_reader(reader)?;
    Ok(res)
}


pub fn generate_file(
    schema_base_path: &str,
    enums_mod: &mut BufWriter<File>,
    base_path: &str,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    println!("#########################################");
    println!("Processing {filename:?}");

    let classpath = filename_to_classpath(schema_base_path, &filename);
    println!("{classpath:?}");

    match classpath {
        ClassPath::Enums { class_path } => {
            if let Err(err) = add_enum(
                &class_path,
                &base_path,
                filename
            ) {
                panic!("Failed to write data from {filename}: {err:?}");
            };
            enums_mod.write_fmt(format_args!("pub mod {class_path};\n"))?;
        }
        ClassPath::Unknown => {
            eprintln!("Nothing to do yet with {filename:?}!");
        }
    }

    Ok(())
}

pub fn generate_scope(base_path: &str) -> Result<(), Box<dyn Error>> {

    let ocsf_dir = format!("{base_path}ocsf/");

    if !PathBuf::from(&ocsf_dir).exists() {
        println!("Dir {ocsf_dir} is missing!");
        panic!();
    }

    let enums_dir = format!("{ocsf_dir}src/enums/");

    if !PathBuf::from(&enums_dir).exists() {
        println!("{enums_dir} is missing, creating it...");
        create_dir_all(&enums_dir)?;
    }

    let enums_mod_file = File::create(PathBuf::from(format!("{enums_dir}mod.rs")))?;
    let mut enums_mod = BufWriter::new(enums_mod_file);

    let mut files = find_files(base_path);
    files.sort();

    for file in files.into_iter() {
        if !file.ends_with(".json") {
            continue;
        }
        if !file.contains("enum") {
            // eprintln!("Skipping {file}");
            continue;
        }
        match generate_file(
            &format!("{base_path}ocsf-schema/"),
            &mut enums_mod,
            &ocsf_dir,
            &file,
        ) {
            Err(err) => println!("Failed to handle {file}: {err:?}"),
            Ok(_) => println!("[OK] {file}"),
        }
    }
    Ok(())
}
