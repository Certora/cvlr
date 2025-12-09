use cvlr_derive::CvlrLog;
enum EnumWithNamed {
    Variant1,
    Variant2 { x: u64 },
    Variant3 { x: u64, y: i32 },
}
impl ::cvlr::log::CvlrLog for EnumWithNamed {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut ::cvlr::log::CvlrLogger) {
        match self {
            EnumWithNamed::Variant1 => {
                logger.log_str(tag, "Variant1");
            }
            EnumWithNamed::Variant2 { ref x } => {
                logger.log_scope_start(tag);
                logger.log_str(tag, "Variant2");
                ::cvlr::log::cvlr_log_with("x", &x, logger);
                logger.log_scope_end(tag);
            }
            EnumWithNamed::Variant3 { ref x, ref y } => {
                logger.log_scope_start(tag);
                logger.log_str(tag, "Variant3");
                ::cvlr::log::cvlr_log_with("x", &x, logger);
                ::cvlr::log::cvlr_log_with("y", &y, logger);
                logger.log_scope_end(tag);
            }
        }
    }
}
fn main() {
    let e1 = EnumWithNamed::Variant1;
    let e2 = EnumWithNamed::Variant2 { x: 42 };
    let e3 = EnumWithNamed::Variant3 {
        x: 10,
        y: -20,
    };
    let mut logger = cvlr::log::CvlrLogger::new();
    e1.log("e1", &mut logger);
    e2.log("e2", &mut logger);
    e3.log("e3", &mut logger);
}
