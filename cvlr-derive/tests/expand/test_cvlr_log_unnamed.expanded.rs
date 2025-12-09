use cvlr_derive::CvlrLog;
struct Tuple(u64, i32, bool);
impl ::cvlr::log::CvlrLog for Tuple {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut ::cvlr::log::CvlrLogger) {
        logger.log_scope_start(tag);
        ::cvlr::log::cvlr_log_with("0", &self.0, logger);
        ::cvlr::log::cvlr_log_with("1", &self.1, logger);
        ::cvlr::log::cvlr_log_with("2", &self.2, logger);
        logger.log_scope_end(tag);
    }
}
fn main() {
    let t = Tuple(1, -2, true);
    let mut logger = cvlr::log::CvlrLogger::new();
    t.log("tuple", &mut logger);
}
