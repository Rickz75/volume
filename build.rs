fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rerun-if-changed=src/native/alsa.c");
        println!("cargo:rerun-if-changed=build.rs");
        if let Err(e) = pkg_config::Config::new().probe("alsa") {
            panic!("{}", e);
        }
        cc::Build::new().file("src/native/alsa.c").compile("alsa");
        println!("cargo:rustc-link-lib=asound");
    }
}
