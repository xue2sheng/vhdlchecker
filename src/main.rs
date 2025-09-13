fn version() -> &'static str {
    "cli x.x.x preprocess ".to_owned() + preprocess::version() + " process x.x.x ai x.x.x"
}
fn main() {
    println!("version: {}", version());
}
