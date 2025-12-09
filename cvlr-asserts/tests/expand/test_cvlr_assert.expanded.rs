use cvlr_asserts::cvlr_assert;
fn main() {
    {
        let c_ = true;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = 1 == 1;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = false;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = x > 0;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
}
