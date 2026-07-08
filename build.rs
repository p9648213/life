fn main() {
    //let generated = generate_r(...);
    //let out = PathBuf::from(env::var("OUT_DIR")?);
    //std::fs::write(out.join("templates.rs"), generated)?;
    //mod templates {
    //   include!(concat!(env!("OUT_DIR"), "/templates.rs"));
    //}
    println!("cargo:rerun-if-changed=templates");
}
