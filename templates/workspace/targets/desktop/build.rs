fn main() {
    {% if desktop-backend == "sdl" %}
    if std::env::var("CARGO_CFG_TARGET_OS").ok().as_deref() == Some("windows") {
        println!("cargo:rustc-link-lib=advapi32");
    }
    {% endif %}
}
