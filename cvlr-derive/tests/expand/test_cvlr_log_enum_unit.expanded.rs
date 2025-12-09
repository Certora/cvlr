use cvlr_derive::CvlrLog;
enum SimpleEnum {
    Variant1,
    Variant2,
}
impl ::cvlr::log::CvlrLog for SimpleEnum {
    fn log(&self, tag: &str, logger: &mut ::cvlr::log::CvlrLogger) {
        match self {
            SimpleEnum::Variant1 => {
                logger.log_str(tag, "Variant1");
            }
            SimpleEnum::Variant2 => {
                logger.log_str(tag, "Variant2");
            }
        }
    }
}
fn main() {
    let e1 = SimpleEnum::Variant1;
    let e2 = SimpleEnum::Variant2;
    let mut logger = cvlr::log::CvlrLogger::new();
    e1.log("e1", &mut logger);
    e2.log("e2", &mut logger);
}
