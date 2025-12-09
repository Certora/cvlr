use cvlr_derive::CvlrLog;

#[derive(CvlrLog)]
struct Tuple(u64, i32, bool);

fn main() {
    let t = Tuple(1, -2, true);
    let mut logger = cvlr::log::CvlrLogger::new();
    t.log("tuple", &mut logger);
}
