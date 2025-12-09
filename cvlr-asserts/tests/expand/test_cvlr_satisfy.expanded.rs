use cvlr_asserts::cvlr_satisfy;
fn main() {
    {
        let c_ = true;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_satisfy_checked(c_);
    };
    {
        let c_ = x == y;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_satisfy_checked(c_);
    };
    {
        let c_ = z > 0;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_satisfy_checked(c_);
    };
}
