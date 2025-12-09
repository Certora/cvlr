use cvlr_derive::CvlrLog;

#[derive(CvlrLog)]
struct UnitStruct;

fn main() {
    let u = UnitStruct;
    let mut logger = cvlr::log::CvlrLogger::new();
    u.log("unit", &mut logger);
}
