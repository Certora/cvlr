use cvlr_derive::CvlrLog;

#[derive(CvlrLog)]
enum EnumWithNamed {
    Variant1,
    Variant2 { x: u64 },
    Variant3 { x: u64, y: i32 },
}

fn main() {
    let e1 = EnumWithNamed::Variant1;
    let e2 = EnumWithNamed::Variant2 { x: 42 };
    let e3 = EnumWithNamed::Variant3 { x: 10, y: -20 };
    let mut logger = cvlr::log::CvlrLogger::new();
    e1.log("e1", &mut logger);
    e2.log("e2", &mut logger);
    e3.log("e3", &mut logger);
}
