use cvlr_log::{cvlr_log, CvlrLogger};
fn main() {
    let mut logger = CvlrLogger::new();
    ::cvlr_log::cvlr_log_with("value", &(42), &mut logger);
    ::cvlr_log::cvlr_log_with("string", &("test"), &mut logger);
    ::cvlr_log::cvlr_log_with("flag", &(true), &mut logger);
}
