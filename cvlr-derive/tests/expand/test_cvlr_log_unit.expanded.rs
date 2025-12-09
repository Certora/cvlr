use cvlr_derive::CvlrLog;
struct UnitStruct;
impl ::cvlr::log::CvlrLog for UnitStruct {
    fn log(&self, tag: &str, logger: &mut ::cvlr::log::CvlrLogger) {
        logger.log_scope_start(tag);
        logger.log_scope_end(tag);
    }
}
fn main() {
    let u = UnitStruct;
    let mut logger = cvlr::log::CvlrLogger::new();
    u.log("unit", &mut logger);
}
