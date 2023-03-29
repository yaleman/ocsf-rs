use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;

use codegen::{Enum, Function, Scope, Variant};
use itertools::{any, Itertools};
use log::{debug, error, info, trace};

use crate::enums::EnumData;
use crate::{
    collapsed_title_case, read_file_to_value, write_source_file, CustomScopeThings, DirPaths,
};

#[derive(Debug)]
pub struct ModuleEnumWithU8 {
    name: String,
    pub variants: HashMap<u8, EnumData>,
}

impl ModuleEnumWithU8 {
    pub fn new(paths: &DirPaths, name: String) -> Self {
        let variants = Self::get_enum_defaults(paths).unwrap();

        ModuleEnumWithU8 { name, variants }
    }

    pub fn get_enum_defaults(
        paths: &DirPaths,
    ) -> Result<HashMap<u8, EnumData>, Box<dyn std::error::Error>> {
        let defaults = format!("{}enums/defaults.json", paths.schema_path);
        trace!("Pulling defaults from {defaults}");
        let defaults: crate::enums::EnumFile =
            serde_json::from_value(read_file_to_value(&defaults)?)?;
        let defaults = defaults.elements;

        trace!("Defaults: {defaults:#?}");
        Ok(defaults)
    }

    pub fn add_to_scope(&self, scope: &mut Scope) {
        let mut new_enum = Enum::new(&self.name);
        new_enum
            .vis("pub")
            .derive("Debug")
            .derive("serde::Serialize")
            .derive("serde::Deserialize");

        let mut enum_to_u8 = Function::new("from");
        enum_to_u8.arg("input", &self.name).ret("u8");
        enum_to_u8.line("match input {");

        let mut try_u8_to_enum = Function::new("try_from");
        try_u8_to_enum
            .arg("input", "u8")
            .ret("Result<Self, String>");
        try_u8_to_enum.line("let res = match input {");

        self.variants.keys().sorted().for_each(|key| {
            let value = self.variants.get(key).unwrap();
            let variant_name = collapsed_title_case(&value.caption);
            let mut variant = Variant::new(&variant_name);
            if let Some(description) = &value.description {
                variant.annotation(&format!("/// {}", description));
            }
            new_enum.push_variant(variant);

            enum_to_u8.line(&format!("    {}::{} => {},", self.name, variant_name, &key));
            try_u8_to_enum.line(&format!("    {} => {}::{},", &key, self.name, variant_name));
        });

        debug!("Adding enum called {} to scope...", &self.name);
        enum_to_u8.line("}");
        try_u8_to_enum.line("_ => return Err(\"invalid value\".to_string()),");
        try_u8_to_enum.line("}; Ok(res)");

        scope.push_enum(new_enum);

        let enum_to_u8_impl = scope.new_impl("u8");
        enum_to_u8_impl.impl_trait(&format!("From<{}>", &self.name));
        enum_to_u8_impl.push_fn(enum_to_u8);

        let try_u8_to_enum_impl = scope.new_impl(&self.name);
        try_u8_to_enum_impl
            .impl_trait("TryFrom<u8>")
            .associate_type("Error", "String");
        try_u8_to_enum_impl.push_fn(try_u8_to_enum);
    }
}

#[derive(Debug)]
pub struct ModuleStruct {
    name: String,
    pub scope: Scope,
}

impl ModuleStruct {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            scope: Scope::new(),
        }
    }
}

#[derive(Debug)]
pub struct Module {
    name: String,
    pub children: HashMap<String, Module>,
    pub enums: Vec<ModuleEnumWithU8>,
    pub structs: Vec<ModuleStruct>,
    pub is_root: bool,
    pub scope: codegen::Scope,
    pub imports: Vec<String>,
}

impl Default for Module {
    fn default() -> Self {
        Module {
            name: "".to_string(),
            children: HashMap::new(),
            enums: vec![],
            structs: vec![],
            is_root: false,
            scope: codegen::Scope::new(),
            imports: vec![],
        }
    }
}

impl Module {
    pub fn new(name: String, is_root: bool) -> Self {
        Self {
            name,
            is_root,
            scope: codegen::Scope::new(),
            ..Default::default()
        }
    }

    /// add an empty child module
    pub fn add_child(&mut self, name: String) {
        self.children.insert(name.clone(), Module::new(name, false));
    }

    /// add an empty struct module
    pub fn add_struct(&mut self, name: impl ToString) {
        self.structs.push(ModuleStruct::new(name));
    }

    pub fn has_enum(&self, name: &str) -> bool {
        if !self.is_root {
            panic!("Only check for enums from the root module please!");
        }
        any(self.enums.iter(), |f| f.name == name)
    }

    pub fn has_struct(&self, name: &str) -> bool {
        any(self.structs.iter(), |f| f.name == name)
    }

    pub fn has_child(&self, name: &str) -> bool {
        self.children.contains_key(name)
    }

    /// add an enum to the module, only do it to the root module though!
    pub fn add_enum(&mut self, name: String) {
        if !self.is_root {
            panic!("Only add enums to the root module please!");
        }
        self.enums.push(ModuleEnumWithU8 {
            name,
            variants: HashMap::new(),
        })
    }

    /// write all the things!
    pub fn write_module(
        &mut self,
        expected_paths: &mut Vec<String>,
        parent_dirname: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let my_filename = parent_dirname.join(format!("{}.rs", self.name));
        debug!("My filename: {:#?}", my_filename);

        if !expected_paths.contains(&my_filename.to_str().unwrap().to_owned()) {
            debug!(
                "Adding expected path from filename {}",
                my_filename.to_str().unwrap()
            );
            expected_paths.push(my_filename.to_str().unwrap().to_owned());
        }

        if !parent_dirname.exists() {
            debug!("Creating directory {:?}", parent_dirname);
            create_dir_all(parent_dirname)?;
        }

        for import in self.imports.iter() {
            self.scope.raw(&format!("{};", import));
        }

        let child_keys: Vec<String> = self.children.keys().cloned().collect();

        info!("Child modules: {child_keys:#?}");

        child_keys.iter().sorted().for_each(|key| {
            if key.is_empty() {
                panic!("Empty module name?");
            }
            self.scope.raw(&format!("pub mod {};", key));
        });

        child_keys.iter().sorted().for_each(|key| {
            let child_path = match self.is_root {
                true => parent_dirname.clone(),
                false => parent_dirname.clone().join(&self.name),
            };

            info!("writing child module '{}' to {:?}", key, child_path);
            let child = self.children.get_mut(key).unwrap();
            // if self.is_root {
            if let Err(err) = child.write_module(expected_paths, &child_path) {
                error!(
                    "Failed to write child module '{}' to {:?}: {:?}",
                    key, parent_dirname, err
                );
            };

            if !expected_paths.contains(&child_path.to_str().unwrap().to_owned()) {
                debug!("Adding expected path {}", child_path.to_str().unwrap());
                expected_paths.push(child_path.to_str().unwrap().to_owned());
            }
        });

        self.enums.iter().for_each(|object| {
            debug!("adding enum to scope {:?}", object.name);
            object.add_to_scope(&mut self.scope)
        });

        if !self.scope.to_string().contains("automatically generated") {
            self.scope.add_generation_timestamp_comment();
        }
        write_source_file(my_filename.to_str().unwrap(), &self.scope.to_string())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_has_child() {
        use super::Module;

        let mut root = Module::new("lib".into(), true);

        assert!(!root.has_child("cheese"));

        root.add_child("cheese".to_string());

        assert!(root.has_child("cheese"));
    }
}
