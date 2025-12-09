use cvlr_log::{cvlr_log, CvlrLogger};

fn main() {
    let mut logger = CvlrLogger::new();
    cvlr_log!(42 => "value"; logger);
    cvlr_log!("test" => "string"; logger);
    cvlr_log!(true => "flag"; logger);
}
