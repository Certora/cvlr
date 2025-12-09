use cvlr_log::cvlr_log;

fn main() {
    cvlr_log!((1 + 2) => "sum");
    cvlr_log!((10 * 5) => "product");
    
    let x = 5;
    let y = 10;
    cvlr_log!((x + y) => "computed");
    cvlr_log!(x, y);
}
