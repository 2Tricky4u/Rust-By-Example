mod hello;
mod primitives;
mod custom_types;
mod variables;

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn main() {
    variables::variables();
}
