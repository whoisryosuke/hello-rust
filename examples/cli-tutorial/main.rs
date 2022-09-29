use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    // Run code using `cargo run --example cli-tutorial -- your flags here`
}
