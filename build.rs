use std::{
    env,
    fs::{self, ReadDir},
    path::PathBuf,
};

fn main() {
    //let generated = generate_r(...);
    //let out = PathBuf::from(env::var("OUT_DIR")?);
    //std::fs::write(out.join("templates.rs"), generated)?;
    //mod templates {
    //   include!(concat!(env!("OUT_DIR"), "/templates.rs"));
    //}
    println!("cargo:rerun-if-changed=templates");
    compile_template();
}

fn compile_template() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let templates_dir = manifest_dir.join("templates");
    let templates = fs::read_dir(&templates_dir).unwrap();
    let mut items = vec![];
    find_all_file(templates, &mut items);
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
            let struct_name = name;

            items.push((full_path, fn_name, struct_name));
        } else {
            find_all_file(path.read_dir().unwrap(), items);
        }
    }
}
