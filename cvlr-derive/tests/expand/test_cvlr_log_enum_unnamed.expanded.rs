use cvlr_derive::CvlrLog;
enum EnumWithUnnamed {
    Variant1,
    Variant2(u64),
    Variant3(u64, i32),
}
impl ::cvlr::log::CvlrLog for EnumWithUnnamed {
    fn log(&self, tag: &str, logger: &mut ::cvlr::log::CvlrLogger) {
        match self {
            EnumWithUnnamed::Variant1 => {
                logger.log_str(tag, "Variant1");
            }
            EnumWithUnnamed::Variant2(ref field0) => {
                logger.log_scope_start(tag);
                logger.log_str(tag, "Variant2");
                ::cvlr::log::cvlr_log_with("0", &field0, logger);
                logger.log_scope_end(tag);
            }
            EnumWithUnnamed::Variant3(ref field0, ref field1) => {
                logger.log_scope_start(tag);
                logger.log_str(tag, "Variant3");
                ::cvlr::log::cvlr_log_with("0", &field0, logger);
                ::cvlr::log::cvlr_log_with("1", &field1, logger);
                logger.log_scope_end(tag);
            }
        }
    }
}
fn main() {
    let e1 = EnumWithUnnamed::Variant1;
    let e2 = EnumWithUnnamed::Variant2(42);
    let e3 = EnumWithUnnamed::Variant3(10, -20);
    let mut logger = cvlr::log::CvlrLogger::new();
    e1.log("e1", &mut logger);
    e2.log("e2", &mut logger);
    e3.log("e3", &mut logger);
}
