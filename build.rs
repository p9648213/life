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

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let templates_dir = manifest_dir.join("templates");
    let templates = fs::read_dir(&templates_dir).unwrap();
    read_all_file(templates);
}

fn read_all_file(read_dir: ReadDir) {
    for item in read_dir {
        let entry = item.unwrap();
        let path = entry.path();
        if entry.file_type().unwrap().is_file() {
            let full_path = path.display().to_string();
            let mut fn_name = full_path
                .split_once("/templates/")
                .unwrap()
                .1
                .split_once(".")
                .unwrap()
                .0
                .replace("/", "_");
            fn_name = format!("render_{}", fn_name);
            println!("cargo::warning={}", fn_name);
        } else {
            read_all_file(path.read_dir().unwrap());
        }
    }
}
