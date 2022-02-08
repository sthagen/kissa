fn main() {
    if let Err(e) = kissa::get_args().and_then(kissa::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
