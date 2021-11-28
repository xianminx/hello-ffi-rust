fn main() {
    cc::Build::new()
        .file("src/extlib.c")
        .shared_flag(true)
        .compile("extlib");
}