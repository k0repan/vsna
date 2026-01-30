mod config;

fn main() {
    let config: config::Config = config::Config::new();
    println!("{config:?}");
}
