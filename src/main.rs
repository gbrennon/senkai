fn main() {
    let args: Vec<String> = std::env::args().collect();
    let app = senkai::build_app();
    app.run(&args);
    std::process::exit(0);
}
