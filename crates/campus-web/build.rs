mod build_env;

fn main() {
    build_env::emit_build_env_vars();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build_env.rs");
}
