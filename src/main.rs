mod hello;
mod primitives;
mod custom_types;
mod variables;
mod types;
mod conversion;
mod flow_control;

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn main() {
    flow_control::whilelet();
}
