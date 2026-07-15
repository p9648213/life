use std::{
    env,
    fs::{self, ReadDir},
    path::PathBuf,
};

use htmlc::compiler::generate_code;

fn main() {
    println!("cargo:rerun-if-changed=templates");
    compile_template();
}

fn compile_template() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let templates_dir = manifest_dir.join("templates");
    let templates = fs::read_dir(&templates_dir).unwrap();
    let mut items = vec![];
    find_template(templates, &mut items);
    let mut code = String::new();
    for (full_path, fn_name, struct_name) in items {
        let html_template = std::fs::read_to_string(full_path).unwrap();
        code.push_str(&generate_code(&html_template, &fn_name, &struct_name).unwrap());
        code.push_str("\n\n");
    }
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    std::fs::write(out.join("templates.rs"), code).unwrap();
}

fn find_template(read_dir: ReadDir, items: &mut Vec<(String, String, String)>) {
    for item in read_dir {
        let entry = item.unwrap();
        let path = entry.path();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file()
            && path
                .extension()
                .is_some_and(|extension| extension == "html")
        {
            let full_path = path.to_str().unwrap();
            let name = full_path
                .split_once("/templates/")
                .and_then(|(_, path)| path.strip_suffix(".html"))
                .unwrap();
            if let Some(ch) = name.chars().find(|ch| {
                !ch.is_ascii_lowercase() && !ch.is_ascii_digit() && *ch != '_' && *ch != '/'
            }) {
                panic!(
                    "Unsupported template path `{name}`: invalid character `{ch}`; \
         each path component may contain only lowercase ASCII letters, digits, and underscores"
                );
            }
            let fn_name = name.replace("/", "_");
            let mut struct_name = String::new();
            let filter_struct_name = name.replace("_", "");
            for name_part in filter_struct_name
                .split('/')
                .filter(|part| !part.is_empty())
            {
                struct_name.push_str(&capitalize_first(name_part));
            }
            items.push((full_path.to_owned(), fn_name, struct_name));
        } else if file_type.is_dir() {
            find_template(path.read_dir().unwrap(), items);
        }
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
