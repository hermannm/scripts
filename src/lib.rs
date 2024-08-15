use std::env;

pub fn init_logger() {
    // We want to enable backtraces by default. `env::var` returns `Err` if env var is not set, so
    // we check that.
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "1")
    }

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .init();
}
