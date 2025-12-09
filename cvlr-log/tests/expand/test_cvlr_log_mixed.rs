use cvlr_log::cvlr_log;

fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    
    // Test: first labeled, rest unlabeled
    cvlr_log!(a => "a", b);
    cvlr_log!(a => "a", b, c);
    
    // Test: first unlabeled, rest labeled
    cvlr_log!(a, b => "b");
    cvlr_log!(a, b => "b", c => "c");
    
    // Test: mixed throughout
    cvlr_log!(a => "a", b, c => "c");
    cvlr_log!(a, b => "b", c);
}
