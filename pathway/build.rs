/// This file serves as a build dependency configuration
/// and a mechanism to enforce cargo reruns when certain files change.
fn main() {
    // Rerun when migrations change
    println!("cargo:rerun-if-changed=./migrations");
}
