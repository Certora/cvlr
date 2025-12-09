use cvlr_log::cvlr_log;
fn main() {
    ::cvlr_log::cvlr_log("u8", &(1u8));
    ::cvlr_log::cvlr_log("u16", &(2u16));
    ::cvlr_log::cvlr_log("u32", &(3u32));
    ::cvlr_log::cvlr_log("u64", &(4u64));
    ::cvlr_log::cvlr_log("usize", &(5usize));
    ::cvlr_log::cvlr_log("u128", &(6u128));
    ::cvlr_log::cvlr_log("i8", &(-1i8));
    ::cvlr_log::cvlr_log("i16", &(-2i16));
    ::cvlr_log::cvlr_log("i32", &(-3i32));
    ::cvlr_log::cvlr_log("i64", &(-4i64));
    ::cvlr_log::cvlr_log("i128", &(-5i128));
}
