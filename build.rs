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
    find_all_file(templates, &mut items);
    let mut code = String::new();
    for (full_path, fn_name, struct_name) in items {
        let html_template = std::fs::read_to_string(full_path).unwrap();
        code.push_str(&generate_code(&html_template, &fn_name, &struct_name).unwrap());
        code.push_str("\n\n");
    }
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    std::fs::write(out.join("templates.rs"), code).unwrap();
}

fn find_all_file(read_dir: ReadDir, items: &mut Vec<(String, String, String)>) {
    for item in read_dir {
        let entry = item.unwrap();
        let path = entry.path();
        if entry.file_type().unwrap().is_file() && path.extension().unwrap() == "html" {
            let full_path = path.display().to_string();
            let name = full_path
                .split_once("/templates/")
                .unwrap()
                .1
                .split_once(".html")
                .unwrap()
                .0;
            let fn_name = name.replace("/", "_");
            if let Some(struct_name) = name.split_once("/") {
                let struct_name = format!(
                    "{}{}",
                    capitalize_first(struct_name.0),
                    capitalize_first(struct_name.1)
                );
                items.push((full_path, fn_name, struct_name));
            } else {
                let struct_name = name.to_owned();
                items.push((full_path, fn_name, struct_name));
            }

        } else {
            find_all_file(path.read_dir().unwrap(), items);
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
