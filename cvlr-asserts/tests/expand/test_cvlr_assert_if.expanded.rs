use cvlr_asserts::cvlr_assert_if;
fn main() {
    if x > 0 {
        {
            let c_ = y > 0;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    }
    if flag {
        {
            let c_ = value == expected;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    }
}
