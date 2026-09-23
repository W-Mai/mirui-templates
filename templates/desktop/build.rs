{% if backend == "sdl" %}fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").ok().as_deref() == Some("windows") {
        println!("cargo:rustc-link-lib=advapi32");
    }
}
{% else %}fn main() {}
{% endif -%}
