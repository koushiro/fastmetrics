fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/COMMIT_EDITMSG");
    built::write_built_file().expect("Failed to acquire build-time information");
}
