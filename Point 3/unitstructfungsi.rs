struct Logger;

fn log_message(_: Logger, msg: &str) {
    println!("Log: {}", msg);
}

fn main() {
    let logger = Logger;
    log_message(logger, "This is a log.");
}
