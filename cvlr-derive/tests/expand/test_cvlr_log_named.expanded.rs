use cvlr_derive::CvlrLog;
struct Point {
    x: u64,
    y: u64,
}
impl ::cvlr::log::CvlrLog for Point {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut ::cvlr::log::CvlrLogger) {
        logger.log_scope_start(tag);
        ::cvlr::log::cvlr_log_with("x", &self.x, logger);
        ::cvlr::log::cvlr_log_with("y", &self.y, logger);
        logger.log_scope_end(tag);
    }
}
fn main() {
    let p = Point { x: 1, y: 2 };
    let mut logger = cvlr::log::CvlrLogger::new();
    p.log("point", &mut logger);
}
