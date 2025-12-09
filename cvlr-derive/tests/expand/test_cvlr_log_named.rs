use cvlr_derive::CvlrLog;

#[derive(CvlrLog)]
struct Point {
    x: u64,
    y: u64,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let mut logger = cvlr::log::CvlrLogger::new();
    p.log("point", &mut logger);
}
