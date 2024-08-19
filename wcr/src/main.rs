fn main() {
    if let Err(e) = wcr::run(wcr::arg()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
