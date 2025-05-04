fn main() {
    cc::Build::new()
        .file("src/monolith_perm.c")
        .compile("monolith_perm");
}