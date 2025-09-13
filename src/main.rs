use lazy_static::lazy_static;

lazy_static! {
    static ref VERSION: String = format!(
        "version: {} {} cli {} preprocess {} process {} ai {}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH"),
        cli::version(),
        preprocess::version(),
        process::version(),
        ai::version()
    );
}

fn main() {
    println!("{}", &*VERSION);
}
