use cvlr_derive::CvlrLog;

#[derive(CvlrLog)]
enum EnumWithUnnamed {
    Variant1,
    Variant2(u64),
    Variant3(u64, i32),
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
