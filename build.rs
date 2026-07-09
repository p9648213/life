use std::{env, fs, path::PathBuf};

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

    for entry in templates {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            file.push(entry);
        } else {
            todo!()
        }
    }
}
