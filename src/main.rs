use lazy_static::lazy_static;

lazy_static! {
    static ref VERSION: String = format!(
        "version: 0.0.1 cli {} preprocess {} process {} ai {}",
        cli::version(),
        preprocess::version(),
        process::version(),
        ai::version()
    );
}

fn main() {
    println!("{}", &*VERSION);
}
