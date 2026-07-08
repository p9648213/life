fn main() {
    println!("cargo:rerun-if-changed=templates");

    let a = template_compiler::hello();
    println!("cargo:warning={a}");
}
